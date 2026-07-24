//! Actor for transmitting APS data frames.

use std::collections::VecDeque;

use bytes::Bytes;
use log::{debug, warn};
use tokio::spawn;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::oneshot::channel;
use zb_aps::{Control, Data};
use zb_core::Destination;
use zb_hw::Ncp;

pub use self::message::Message;
pub use self::metadata::Metadata;
use crate::MPSC_CHANNEL_SIZE;

mod message;
mod metadata;

const INITIAL_COUNTER: u8 = 0;

type PendingResponse = tokio::sync::oneshot::Sender<Result<(), zb_hw::Error>>;

/// Coordinator-internal APS transmission API.
pub trait Aps {
    /// Transmit an APS frame and, when it requests an acknowledgement, await the hardware result.
    fn transmit(
        &self,
        destination: Destination,
        frame: Data<Bytes>,
    ) -> impl Future<Output = Result<(), zb_hw::Error>> + Send;
}

impl Aps for Sender<Message> {
    async fn transmit(
        &self,
        destination: Destination,
        frame: Data<Bytes>,
    ) -> Result<(), zb_hw::Error> {
        let acknowledged = frame.header().control().contains(Control::ACK_REQUEST);
        let (response, result) = if acknowledged {
            let (response, result) = channel();
            (Some(response), Some(result))
        } else {
            (None, None)
        };

        self.send(Message::Transmit {
            destination,
            frame,
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

    fn take_response(&mut self, counter: u8) -> Option<PendingResponse> {
        let index = self
            .responses
            .iter()
            .position(|(pending_counter, _)| *pending_counter == counter)?;
        self.responses.remove(index).map(|(_, response)| response)
    }

    fn handle_response(&mut self, response: Result<u8, zb_hw::Error>) {
        match response {
            Ok(counter) => {
                let Some(sender) = self.take_response(counter) else {
                    warn!("Received APS response for unknown counter: {counter}");
                    return;
                };
                sender.send(Ok(())).unwrap_or_else(drop);
            }
            Err(error) => {
                let Some((_, sender)) = self.responses.pop_front() else {
                    warn!("Received APS transmission error without a pending response: {error}");
                    return;
                };
                sender.send(Err(error)).unwrap_or_else(drop);
            }
        }
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
                    frame,
                    response,
                } => {
                    let (mut header, payload) = frame.into_parts();
                    let counter = self.next_counter();
                    header.set_counter(counter);
                    let frame = Data::new(header, payload);

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
                Message::ApsResponse { response } => {
                    self.handle_response(response);
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
    pub fn spawn(ncp: T) -> Sender<Message> {
        let (aps_tx, aps_rx) = tokio::sync::mpsc::channel(MPSC_CHANNEL_SIZE);
        spawn(Self::new(ncp).run(aps_rx));
        aps_tx
    }
}

#[cfg(test)]
mod tests {
    use bytes::Bytes;
    use tokio::runtime::Runtime;
    use tokio::sync::mpsc::channel;
    use zb_aps::{Data, TxOptions};
    use zb_core::destination::{Broadcast, Destination};
    use zb_core::{Endpoint, Profile, short_id};

    use super::{Aps, INITIAL_COUNTER, Message, Transceiver};
    use crate::aps::Metadata;

    const CHANNEL_SIZE: usize = 1;
    const CLUSTER_ID: u16 = 0x1234;
    const LAST_COUNTER: u8 = u8::MAX;

    fn destination() -> Destination {
        Broadcast::new(short_id::Broadcast::AllDevices, Endpoint::Broadcast).into()
    }

    fn frame(tx_options: TxOptions) -> Data<Bytes> {
        Metadata::new(Profile::ZigbeeHomeAutomation, CLUSTER_ID)
            .with_tx_options(tx_options)
            .frame(destination(), Bytes::new())
    }

    #[test]
    fn omits_response_for_unacknowledged_transmission() {
        Runtime::new()
            .expect("runtime must be available")
            .block_on(async {
                let (sender, mut receiver) = channel(CHANNEL_SIZE);

                sender
                    .transmit(destination(), frame(TxOptions::empty()))
                    .await
                    .expect("APS actor channel must be available");

                let Message::Transmit { response, .. } =
                    receiver.recv().await.expect("message must be available")
                else {
                    panic!("expected APS transmit message");
                };
                assert!(response.is_none());
            });
    }

    #[test]
    fn awaits_hardware_response_for_acknowledged_transmission() {
        Runtime::new()
            .expect("runtime must be available")
            .block_on(async {
                let (sender, mut receiver) = channel(CHANNEL_SIZE);
                let task = tokio::spawn(async move {
                    sender
                        .transmit(destination(), frame(TxOptions::ACKNOWLEDGED_TRANSMISSION))
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
    fn transmitted_frame_counter_can_be_replaced() {
        let frame = frame(TxOptions::empty());
        let (mut header, payload) = frame.into_parts();
        header.set_counter(LAST_COUNTER);
        let frame = Data::new(header, payload);

        assert_eq!(frame.header().counter(), LAST_COUNTER);
    }

    #[test]
    fn successful_event_resolves_matching_counter() {
        Runtime::new()
            .expect("runtime must be available")
            .block_on(async {
                let mut transceiver = Transceiver::new(());
                let (response, result) = tokio::sync::oneshot::channel();
                transceiver.responses.push_back((LAST_COUNTER, response));

                transceiver.handle_response(Ok(LAST_COUNTER));

                assert!(result.await.expect("response must be available").is_ok());
                assert!(transceiver.responses.is_empty());
            });
    }

    #[test]
    fn error_event_resolves_oldest_pending_response() {
        Runtime::new()
            .expect("runtime must be available")
            .block_on(async {
                let mut transceiver = Transceiver::new(());
                let (first_response, first_result) = tokio::sync::oneshot::channel();
                let (second_response, _second_result) = tokio::sync::oneshot::channel();
                transceiver
                    .responses
                    .push_back((INITIAL_COUNTER, first_response));
                transceiver
                    .responses
                    .push_back((LAST_COUNTER, second_response));

                transceiver.handle_response(Err(zb_hw::Error::NotImplemented));

                assert!(matches!(
                    first_result.await.expect("response must be available"),
                    Err(zb_hw::Error::NotImplemented)
                ));
                assert_eq!(transceiver.responses.len(), CHANNEL_SIZE);
                assert_eq!(
                    transceiver.responses.front().map(|(counter, _)| *counter),
                    Some(LAST_COUNTER)
                );
            });
    }
}
