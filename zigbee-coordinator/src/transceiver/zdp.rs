//! Transceiver to send and receive ZDP messages.

use std::collections::BTreeMap;

use le_stream::ToLeStream;
use log::{error, warn};
use tokio::sync::mpsc::Receiver;
use tokio::sync::oneshot::Sender;
use zdp::Command;
use zigbee::{Address, Endpoint};
use zigbee_hw::{Event, Metadata, Ncp};

pub use self::handle::Handle;
pub use self::message::Message;

mod handle;
mod message;

/// Zigbee transceiver actor.
#[derive(Debug)]
pub struct Transceiver<T> {
    ncp: T,
    responses: BTreeMap<u8, Sender<Command>>,
    seq: u8,
}

impl<T> Transceiver<T> {
    /// Crate a new transceiver.
    #[must_use]
    pub const fn new(ncp: T) -> Self {
        Self {
            ncp,
            responses: BTreeMap::new(),
            seq: 0,
        }
    }
}

impl<T> Transceiver<T>
where
    T: Ncp,
{
    /// Run the transceiver.
    pub async fn run(mut self, mut messages: Receiver<Message>) {
        while let Some(message) = messages.recv().await {
            match message {
                Message::Event(event) => self.handle_event(event),
                Message::Unicast {
                    address,
                    endpoint,
                    metadata,
                    command,
                    response,
                } => {
                    self.unicast(address, endpoint, metadata, *command, response)
                        .await;
                }
                Message::Subscribe { seq, response } => {
                    self.responses.insert(seq, response);
                }
            }
        }
    }

    /// Return and increment the ZCL sequence number.
    const fn next_seq(&mut self) -> u8 {
        let seq = self.seq;
        self.seq = self.seq.wrapping_add(1);
        seq
    }

    fn handle_event(&mut self, event: Event) {
        match event {
            Event::MessageReceived { aps_frame, .. } => {
                let (_aps_header, payload) = aps_frame.into_parts();

                if let zigbee_hw::Command::Zdp(frame) = payload {
                    let (seq, command) = frame.into_parts();

                    if let Some(sender) = self.responses.remove(&seq) {
                        sender.send(command).unwrap_or_else(|error| {
                            error!("Failed to send ZCL response: {error:?}");
                        });
                    }
                }
            }
            other => warn!("Unhandled ZCL event: {other:?}"),
        }
    }

    /// Send a unicast message.
    async fn unicast(
        &mut self,
        address: Address,
        endpoint: Endpoint,
        metadata: Metadata,
        command: Command,
        response: Sender<Result<(), zigbee_hw::Error>>,
    ) {
        let aps_frame = self.make_aps_frame(metadata, command);
        let result = self.ncp.unicast(address, endpoint, aps_frame).await;
        response.send(result.map(drop)).unwrap_or_else(|error| {
            error!("Failed to send unicast response: {error:?}");
        });
    }

    /// Create a new APS frame.
    fn make_aps_frame(&mut self, metadata: Metadata, command: Command) -> zigbee_hw::Frame {
        let payload = self.make_zdp_frame(command).to_le_stream().collect();

        #[expect(unsafe_code)]
        // SAFETY: We trust the caller that the given metadata and payload match.
        unsafe {
            zigbee_hw::Frame::new(metadata, payload)
        }
    }

    /// Create a new ZDP frame.
    fn make_zdp_frame(&mut self, command: Command) -> zdp::Frame<Command> {
        zdp::Frame::new(self.next_seq(), command)
    }
}
