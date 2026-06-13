//! Transceiver to send and receive ZCL messages.

use std::collections::BTreeMap;

use le_stream::ToLeStream;
use log::{error, warn};
use tokio::sync::mpsc::Receiver;
use tokio::sync::oneshot;
use tokio::sync::oneshot::{Sender, channel};
use zcl::{Cluster, CommandDispatch};
use zigbee::Endpoint;
use zigbee_hw::{Command, Event, Metadata, Ncp};

pub use self::handle::Handle;
pub use self::message::{Message, Payload};

mod handle;
mod message;

/// Zigbee transceiver actor.
#[derive(Debug)]
pub struct Transceiver<T> {
    ncp: T,
    responses: BTreeMap<u8, Sender<Cluster>>,
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
                Message::Event(event) => self.handle_event(event),
                Message::Unicast {
                    short_id,
                    endpoint,
                    payload,
                    response,
                } => {
                    self.unicast(short_id, endpoint, *payload, response).await;
                }
                Message::Communicate {
                    short_id,
                    endpoint,
                    payload,
                    response,
                } => {
                    self.communicate(short_id, endpoint, *payload, response)
                        .await;
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

                if let Command::Zcl(frame) = payload {
                    let (header, payload) = frame.into_parts();

                    if let Some(sender) = self.responses.remove(&header.seq()) {
                        sender.send(payload).unwrap_or_else(|error| {
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
        short_id: u16,
        endpoint: Endpoint,
        frame: Payload<Cluster>,
        response: Sender<Result<(), zigbee_hw::Error>>,
    ) {
        let (metadata, manufacturer_code, command) = frame.into_parts();
        let zcl_frame = self.make_zcl_frame(manufacturer_code, command);

        #[expect(unsafe_code)]
        // SAFETY: We extracted the metadata and command from the payload above
        // and created a valid ZCL frame from that command.
        // Hence, the resulting metadata and payload match.
        let aps_frame = unsafe { Self::make_aps_frame(metadata, zcl_frame) };

        let result = self.ncp.unicast(short_id, endpoint, aps_frame).await;
        response.send(result.map(drop)).unwrap_or_else(|error| {
            error!("Failed to send unicast response: {error:?}");
        });
    }

    /// Send a unicast message with back-channel communication.
    async fn communicate(
        &mut self,
        short_id: u16,
        endpoint: Endpoint,
        frame: Payload<Cluster>,
        response: Sender<Result<oneshot::Receiver<Cluster>, zigbee_hw::Error>>,
    ) {
        let (metadata, manufacturer_code, command) = frame.into_parts();
        let zcl_frame = self.make_zcl_frame(manufacturer_code, command);
        let seq = zcl_frame.header().seq();

        #[expect(unsafe_code)]
        // SAFETY: We extracted the metadata and command from the payload above
        // and created a valid ZCL frame from that command.
        // Hence, the resulting metadata and payload match.
        let aps_frame = unsafe { Self::make_aps_frame(metadata, zcl_frame) };

        match self.ncp.unicast(short_id, endpoint, aps_frame).await {
            Ok(_) => {
                let (tx, rx) = channel();
                self.responses.insert(seq, tx);
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
    unsafe fn make_aps_frame(metadata: Metadata, frame: zcl::Frame<Cluster>) -> zigbee_hw::Frame {
        let payload = frame.to_le_stream().collect();

        #[expect(unsafe_code)]
        // SAFETY: We trust the caller that the given metadata and payload match.
        unsafe {
            zigbee_hw::Frame::new(metadata, payload)
        }
    }

    /// Create a new ZCL frame.
    fn make_zcl_frame(
        &mut self,
        manufacturer_code: Option<u16>,
        command: Cluster,
    ) -> zcl::Frame<Cluster> {
        let header = zcl::Header::new(
            command.scope(),
            command.direction(),
            command.disable_default_response(),
            manufacturer_code,
            self.next_seq(),
            command.command_id(),
        );

        #[expect(unsafe_code)]
        // SAFETY: We constructed the ZCL header from the associated data of the frame above,
        // and hence the resulting frame is valid.
        unsafe {
            zcl::Frame::new_unchecked(header, command)
        }
    }
}
