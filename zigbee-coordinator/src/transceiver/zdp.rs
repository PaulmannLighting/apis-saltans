//! Transceiver to send and receive ZDP messages.

use std::collections::BTreeMap;

use le_stream::ToLeStream;
use log::error;
use tokio::sync::mpsc::Receiver;
use tokio::sync::oneshot;
use tokio::sync::oneshot::{Sender, channel};
use zdp::{Command, Frame};
use zigbee::Endpoint;
use zigbee_hw::{Metadata, Ncp};

pub use self::handle::Handle;
pub use self::message::{Message, Payload};

mod handle;
mod message;

/// Zigbee transceiver actor.
#[derive(Debug)]
pub struct Transceiver<T> {
    ncp: T,
    responses: BTreeMap<(u8, u16), Sender<Command>>,
    seq: u8,
}

impl<T> Transceiver<T> {
    /// Create a new transceiver.
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
                Message::Received { src_address, frame } => {
                    self.handle_message_received(src_address, *frame);
                }
                Message::Communicate {
                    short_id,
                    payload,
                    response,
                } => {
                    self.communicate(short_id, *payload, response).await;
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

    fn handle_message_received(&mut self, src_address: u16, frame: Frame<Command>) {
        let (seq, command) = frame.into_parts();

        if let Some(sender) = self.responses.remove(&(seq, src_address)) {
            sender.send(command).unwrap_or_else(|error| {
                error!("Failed to send ZDP response: {error:?}");
            });
        }
    }

    /// Send a unicast message with back-channel communication.
    async fn communicate(
        &mut self,
        short_id: u16,
        payload: Payload<Command>,
        response: Sender<Result<oneshot::Receiver<Command>, zigbee_hw::Error>>,
    ) {
        let (metadata, command) = payload.into_parts();
        let zdp_frame = self.make_zdp_frame(command);
        let seq = zdp_frame.seq();

        #[expect(unsafe_code)]
        // SAFETY: We extracted the metadata and command from the payload above
        // and created a valid ZDP frame from that command.
        // Hence, the resulting metadata and payload match.
        let aps_frame = unsafe { Self::make_aps_frame(metadata, zdp_frame) };

        match self.ncp.unicast(short_id, Endpoint::Data, aps_frame).await {
            Ok(_) => {
                let (tx, rx) = channel();
                self.responses.insert((seq, short_id), tx);
                response.send(Ok(rx))
            }
            Err(error) => response.send(Err(error)),
        }
        .unwrap_or_else(|error| {
            error!("Failed to send unicast response: {error:?}");
        });
    }

    /// Create a new APS frame.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the given metadata and payload match.
    #[expect(unsafe_code)]
    unsafe fn make_aps_frame(metadata: Metadata, frame: Frame<Command>) -> zigbee_hw::Frame {
        let payload = frame.to_le_stream().collect();

        #[expect(unsafe_code)]
        // SAFETY: We trust that the caller upholds the safety invariants mentioned above.
        unsafe {
            zigbee_hw::Frame::new(metadata, payload)
        }
    }

    /// Create a new ZDP frame.
    const fn make_zdp_frame(&mut self, command: Command) -> Frame<Command> {
        Frame::new(self.next_seq(), command)
    }
}
