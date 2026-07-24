//! Actor for transmitting APS data frames.

use std::collections::VecDeque;

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
            .map_err(|_| zb_hw::Error::DriverSend)?;

        if let Some(result) = result {
            result.await?
        } else {
            Ok(())
        }
    }

    /// Forward a hardware APS acknowledgement to the APS actor.
    pub async fn ack(&self, sequence: u8) -> Result<(), zb_hw::Error> {
        self.0
            .send(Message::Ack { sequence })
            .await
            .map_err(|_| zb_hw::Error::DriverSend)
    }

    /// Forward a hardware APS negative acknowledgement to the APS actor.
    pub async fn nak(&self, sequence: u8, error: zb_hw::Error) -> Result<(), zb_hw::Error> {
        self.0
            .send(Message::Nak { sequence, error })
            .await
            .map_err(|_| zb_hw::Error::DriverSend)
    }
}

/// APS transmission actor.
#[derive(Debug)]
pub struct Transceiver<T> {
    ncp: T,
    counter: u8,
    responses: VecDeque<(u8, PendingResponse)>,
}

impl<T> Transceiver<T> {
    /// Create an APS actor with its frame counter initialized to zero.
    #[must_use]
    pub const fn new(ncp: T) -> Self {
        Self {
            ncp,
            counter: INITIAL_COUNTER,
            responses: VecDeque::new(),
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

    fn take_response(&mut self, counter: u8) -> Option<PendingResponse> {
        let index = self
            .responses
            .iter()
            .position(|(pending_counter, _)| *pending_counter == counter)?;
        self.responses.remove(index).map(|(_, response)| response)
    }

    fn handle_ack(&mut self, sequence: u8) {
        let Some(sender) = self.take_response(sequence) else {
            warn!("Received APS acknowledgement for unknown sequence: {sequence}");
            return;
        };
        sender.send(Ok(())).unwrap_or_else(drop);
    }

    fn handle_nak(&mut self, sequence: u8, error: zb_hw::Error) {
        let Some(sender) = self.take_response(sequence) else {
            warn!("Received APS negative acknowledgement for unknown sequence {sequence}: {error}");
            return;
        };
        sender.send(Err(error)).unwrap_or_else(drop);
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
                    let (counter, frame) = self.make_frame(destination, metadata, payload);

                    if let Some(response) = response {
                        self.responses.push_back((counter, response));
                    }

                    if let Err(error) = self.ncp.transmit(destination, frame).await {
                        if let Some(response) = self.take_response(counter) {
                            response.send(Err(error)).unwrap_or_else(drop);
                        } else {
                            debug!("Failed to send APS frame to hardware actor: {error:?}");
                        }
                    }
                }
                Message::Ack { sequence } => {
                    self.handle_ack(sequence);
                }
                Message::Nak { sequence, error } => {
                    self.handle_nak(sequence, error);
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

    use super::{Aps, INITIAL_COUNTER, Message, Transceiver};
    use crate::aps::Metadata;

    const CHANNEL_SIZE: usize = 1;
    const CLUSTER_ID: u16 = 0x1234;
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
    fn successful_event_resolves_matching_counter() {
        Runtime::new()
            .expect("runtime must be available")
            .block_on(async {
                let mut transceiver = Transceiver::new(());
                let (response, result) = tokio::sync::oneshot::channel();
                transceiver.responses.push_back((LAST_COUNTER, response));

                transceiver.handle_ack(LAST_COUNTER);

                assert!(result.await.expect("response must be available").is_ok());
                assert!(transceiver.responses.is_empty());
            });
    }

    #[test]
    fn negative_acknowledgement_resolves_matching_sequence() {
        Runtime::new()
            .expect("runtime must be available")
            .block_on(async {
                let mut transceiver = Transceiver::new(());
                let (first_response, _first_result) = tokio::sync::oneshot::channel();
                let (second_response, second_result) = tokio::sync::oneshot::channel();
                transceiver
                    .responses
                    .push_back((INITIAL_COUNTER, first_response));
                transceiver
                    .responses
                    .push_back((LAST_COUNTER, second_response));

                transceiver.handle_nak(LAST_COUNTER, zb_hw::Error::NotImplemented);

                assert!(matches!(
                    second_result.await.expect("response must be available"),
                    Err(zb_hw::Error::NotImplemented)
                ));
                assert_eq!(transceiver.responses.len(), CHANNEL_SIZE);
                assert_eq!(
                    transceiver.responses.front().map(|(counter, _)| *counter),
                    Some(INITIAL_COUNTER)
                );
            });
    }
}
