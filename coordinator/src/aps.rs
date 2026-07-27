//! Actor for transmitting APS data frames.

use std::collections::BTreeMap;

use bytes::Bytes;
use log::{debug, warn};
use tokio::spawn;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::oneshot::channel;
use zb_aps::data::Header;
use zb_aps::{Data, TxOptions};
use zb_core::Destination;
use zb_hw::Ncp;

pub use self::message::Message;
pub use self::metadata::Metadata;
use crate::MPSC_CHANNEL_SIZE;

mod message;
mod metadata;

const INITIAL_COUNTER: u8 = 0;

type PendingResponse = tokio::sync::oneshot::Sender<Result<(), zb_hw::Error>>;

/// Handle for sending commands to the APS actor.
#[derive(Clone, Debug)]
pub struct Aps(Sender<Message>);

impl Aps {
    /// Wrap an APS actor sender.
    #[must_use]
    pub const fn new(sender: Sender<Message>) -> Self {
        Self(sender)
    }

    /// Transmit an APS frame and, when it requests an acknowledgement, await the hardware result.
    pub async fn transmit(
        &self,
        destination: Destination,
        metadata: Metadata,
        payload: Bytes,
    ) -> Result<(), zb_hw::Error> {
        let (response, result) = if metadata.acknowledged() {
            let (response, result) = channel();
            (Some(response), Some(result))
        } else {
            (None, None)
        };

        self.0
            .send(Message::Transmit {
                destination,
                metadata,
                payload,
                response,
            })
            .await
            .map_err(|_| zb_hw::Error::ActorUnavailable)?;

        if let Some(result) = result {
            result.await?
        } else {
            Ok(())
        }
    }

    /// Forward a hardware APS acknowledgement to the APS actor.
    pub async fn ack(&self, counter: u8) -> Result<(), zb_hw::Error> {
        self.0
            .send(Message::Ack { counter })
            .await
            .map_err(|_| zb_hw::Error::ActorUnavailable)
    }

    /// Forward a failed hardware APS transmission to the APS actor.
    pub async fn nak(
        &self,
        counter: u8,
        error: zb_hw::TransmissionError,
    ) -> Result<(), zb_hw::Error> {
        self.0
            .send(Message::Nak { counter, error })
            .await
            .map_err(|_| zb_hw::Error::ActorUnavailable)
    }
}

/// APS transmission actor.
#[derive(Debug)]
pub struct Transceiver<T> {
    ncp: T,
    counter: u8,
    responses: BTreeMap<u8, PendingResponse>,
}

impl<T> Transceiver<T> {
    /// Create an APS actor with its frame counter initialized to zero.
    #[must_use]
    pub const fn new(ncp: T) -> Self {
        Self {
            ncp,
            counter: INITIAL_COUNTER,
            responses: BTreeMap::new(),
        }
    }

    /// Return and increment the APS frame counter.
    const fn next_counter(&mut self) -> u8 {
        let counter = self.counter;
        self.counter = self.counter.wrapping_add(1);
        counter
    }

    fn make_frame(
        &mut self,
        destination: Destination,
        metadata: Metadata,
        payload: Bytes,
    ) -> (u8, Data<Bytes>) {
        let counter = self.next_counter();
        let mut header = Header::new(
            destination.into(),
            metadata.cluster_id(),
            metadata.profile().into(),
            metadata.source_endpoint(),
            counter,
            None,
        );
        header.set_security(metadata.tx_options().contains(TxOptions::SECURITY_ENABLED));
        header.set_ack_request(metadata.acknowledged());
        let frame = Data::new(header, payload);
        (counter, frame)
    }

    fn expire_pending_response(&mut self, counter: u8) {
        let Some(sender) = self.responses.remove(&counter) else {
            return;
        };
        sender
            .send(Err(zb_hw::TransmissionError::Timeout.into()))
            .unwrap_or_else(drop);
    }

    fn prepare_transmission(
        &mut self,
        destination: Destination,
        metadata: Metadata,
        payload: Bytes,
    ) -> (u8, Data<Bytes>) {
        let (counter, frame) = self.make_frame(destination, metadata, payload);
        self.expire_pending_response(counter);
        (counter, frame)
    }

    fn handle_ack(&mut self, counter: u8) {
        let Some(sender) = self.responses.remove(&counter) else {
            warn!("Received APS acknowledgement for unknown counter: {counter}");
            return;
        };
        sender.send(Ok(())).unwrap_or_else(drop);
    }

    fn handle_nak(&mut self, counter: u8, error: zb_hw::TransmissionError) {
        let Some(sender) = self.responses.remove(&counter) else {
            warn!("Received APS failure for unknown counter {counter}: {error}");
            return;
        };
        sender.send(Err(error.into())).unwrap_or_else(drop);
    }
}

impl<T> Transceiver<T>
where
    T: Ncp + Sync,
{
    /// Run the APS actor.
    pub async fn run(mut self, mut messages: Receiver<Message>) {
        while let Some(message) = messages.recv().await {
            match message {
                Message::Transmit {
                    destination,
                    metadata,
                    payload,
                    response,
                } => {
                    let (counter, frame) =
                        self.prepare_transmission(destination, metadata, payload);

                    match self.ncp.transmit(destination, frame).await {
                        Ok(()) => {
                            if let Some(response) = response {
                                self.responses.insert(counter, response);
                            }
                        }
                        Err(error) => {
                            if let Some(response) = response {
                                response.send(Err(error)).unwrap_or_else(drop);
                            } else {
                                debug!("Hardware rejected APS frame: {error:?}");
                            }
                        }
                    }
                }
                Message::Ack { counter } => {
                    self.handle_ack(counter);
                }
                Message::Nak { counter, error } => {
                    self.handle_nak(counter, error);
                }
            }
        }
    }
}

impl<T> Transceiver<T>
where
    T: Ncp + Send + Sync + 'static,
{
    /// Spawn the APS actor.
    pub fn spawn(ncp: T) -> Aps {
        let (aps_tx, aps_rx) = tokio::sync::mpsc::channel(MPSC_CHANNEL_SIZE);
        spawn(Self::new(ncp).run(aps_rx));
        Aps::new(aps_tx)
    }
}

#[cfg(test)]
mod tests {
    use bytes::Bytes;
    use tokio::runtime::Runtime;
    use tokio::sync::mpsc::channel;
    use zb_aps::{Control, TxOptions};
    use zb_core::destination::{Broadcast, Destination};
    use zb_core::{Endpoint, Profile, short_id};

    use super::{Aps, Message, Transceiver};
    use crate::aps::Metadata;

    const CHANNEL_SIZE: usize = 1;
    const CLUSTER_ID: u16 = 0x1234;
    const FIRST_COUNTER: u8 = 1;
    const SECOND_COUNTER: u8 = 2;
    const LAST_COUNTER: u8 = u8::MAX;
    const PAYLOAD: &[u8] = &[0x12, 0x34];

    fn destination() -> Destination {
        Broadcast::new(short_id::Broadcast::AllDevices, Endpoint::Broadcast).into()
    }

    const fn metadata(tx_options: TxOptions) -> Metadata {
        Metadata::new(Profile::ZigbeeHomeAutomation, CLUSTER_ID).with_tx_options(tx_options)
    }

    #[test]
    fn omits_response_for_unacknowledged_transmission() {
        Runtime::new()
            .expect("runtime must be available")
            .block_on(async {
                let (sender, mut receiver) = channel(CHANNEL_SIZE);
                let aps = Aps::new(sender);
                let metadata = metadata(TxOptions::empty());

                aps.transmit(destination(), metadata, Bytes::from_static(PAYLOAD))
                    .await
                    .expect("APS actor channel must be available");

                let Message::Transmit {
                    metadata: sent_metadata,
                    payload,
                    response,
                    ..
                } = receiver.recv().await.expect("message must be available")
                else {
                    panic!("expected APS transmit message");
                };
                assert_eq!(sent_metadata, metadata);
                assert_eq!(payload, PAYLOAD);
                assert!(response.is_none());
            });
    }

    #[test]
    fn awaits_hardware_response_for_acknowledged_transmission() {
        Runtime::new()
            .expect("runtime must be available")
            .block_on(async {
                let (sender, mut receiver) = channel(CHANNEL_SIZE);
                let aps = Aps::new(sender);
                let task = tokio::spawn(async move {
                    aps.transmit(
                        destination(),
                        metadata(TxOptions::ACKNOWLEDGED_TRANSMISSION),
                        Bytes::new(),
                    )
                    .await
                });
                let Message::Transmit { response, .. } =
                    receiver.recv().await.expect("message must be available")
                else {
                    panic!("expected APS transmit message");
                };

                response
                    .expect("acknowledged frame must have a response")
                    .send(Ok(()))
                    .expect("APS response receiver must be available");

                assert!(task.await.expect("task must complete").is_ok());
            });
    }

    #[test]
    fn counter_wraps_after_its_maximum_value() {
        let mut transceiver = Transceiver::new(());
        transceiver.counter = LAST_COUNTER;

        assert_eq!(transceiver.next_counter(), LAST_COUNTER);
        assert_eq!(transceiver.next_counter(), 0);
    }

    #[test]
    fn actor_constructs_frame_with_its_counter() {
        let mut transceiver = Transceiver::new(());
        transceiver.counter = LAST_COUNTER;
        let (counter, frame) =
            transceiver.make_frame(destination(), metadata(TxOptions::empty()), Bytes::new());

        assert_eq!(counter, LAST_COUNTER);
        assert_eq!(frame.header().counter(), LAST_COUNTER);
        assert!(!frame.header().control().contains(Control::ACK_REQUEST));
    }

    #[test]
    fn acknowledgement_resolves_matching_transmission() {
        Runtime::new()
            .expect("runtime must be available")
            .block_on(async {
                let mut transceiver = Transceiver::new(());
                let (response, result) = tokio::sync::oneshot::channel();
                transceiver.responses.insert(FIRST_COUNTER, response);

                transceiver.handle_ack(FIRST_COUNTER);

                assert!(result.await.expect("response must be available").is_ok());
                assert!(transceiver.responses.is_empty());
            });
    }

    #[test]
    fn negative_acknowledgement_resolves_matching_transmission() {
        Runtime::new()
            .expect("runtime must be available")
            .block_on(async {
                let mut transceiver = Transceiver::new(());
                let (first_response, _first_result) = tokio::sync::oneshot::channel();
                let (second_response, second_result) = tokio::sync::oneshot::channel();
                transceiver.responses.insert(FIRST_COUNTER, first_response);
                transceiver
                    .responses
                    .insert(SECOND_COUNTER, second_response);

                transceiver.handle_nak(SECOND_COUNTER, zb_hw::TransmissionError::Rejected);

                assert!(matches!(
                    second_result.await.expect("response must be available"),
                    Err(zb_hw::Error::Transmission(
                        zb_hw::TransmissionError::Rejected
                    ))
                ));
                assert_eq!(transceiver.responses.len(), CHANNEL_SIZE);
                assert!(transceiver.responses.contains_key(&FIRST_COUNTER));
            });
    }

    #[test]
    fn reusing_counter_times_out_pending_transmission() {
        Runtime::new()
            .expect("runtime must be available")
            .block_on(async {
                let mut transceiver = Transceiver::new(());
                let (response, result) = tokio::sync::oneshot::channel();
                transceiver.responses.insert(LAST_COUNTER, response);
                transceiver.counter = LAST_COUNTER;

                let (counter, _) = transceiver.prepare_transmission(
                    destination(),
                    metadata(TxOptions::empty()),
                    Bytes::new(),
                );
                assert_eq!(counter, LAST_COUNTER);

                assert!(matches!(
                    result.await.expect("response must be available"),
                    Err(zb_hw::Error::Transmission(
                        zb_hw::TransmissionError::Timeout
                    ))
                ));
                assert!(transceiver.responses.is_empty());
            });
    }
}
