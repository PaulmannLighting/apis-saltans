//! The transmitter represents the main external Zigbee transmitter API.

use std::collections::BTreeMap;

use le_stream::ToLeStream;
use log::{error, warn};
use tokio::sync::mpsc::Receiver;
use tokio::sync::oneshot::Sender;
use zcl::{Cluster, CommandDispatch};
use zigbee::Endpoint;
use zigbee_hw::{Command, Event, Metadata, Ncp};

pub use self::handle::Handle;
pub use self::message::{Message, Payload};

mod handle;
mod message;

/// Zigbee transmitter actor.
#[derive(Debug)]
pub struct Transmitter<T> {
    ncp: T,
    zcl_responses: BTreeMap<u8, Sender<Cluster>>,
    zcl_seq: u8,
    zdp_seq: u8,
}

impl<T> Transmitter<T> {
    /// Crate a new transmitter.
    #[must_use]
    pub const fn new(ncp: T) -> Self {
        Self {
            ncp,
            zcl_responses: BTreeMap::new(),
            zcl_seq: 0,
            zdp_seq: 0,
        }
    }
}

impl<T> Transmitter<T>
where
    T: Ncp,
{
    /// Run the transmitter.
    pub async fn run(mut self, mut messages: Receiver<Message>) {
        while let Some(message) = messages.recv().await {
            match message {
                Message::AllowJoins { duration } => {
                    todo!("Allow joins")
                }
                Message::Unicast {
                    short_id,
                    endpoint,
                    metadata,
                    payload,
                    response,
                } => {
                    let result = self.unicast(short_id, endpoint, metadata, *payload).await;
                    response.send(result.map(drop)).unwrap_or_else(|error| {
                        error!("Failed to send unicast response: {error:?}");
                    });
                }
                Message::Subscribe { seq, response } => {
                    self.zcl_responses.insert(seq, response);
                }
                Message::Event(event) => match event {
                    Event::MessageReceived { aps_frame, .. } => {
                        let (_aps_header, payload) = aps_frame.into_parts();

                        if let Command::Zcl(frame) = payload {
                            let (header, payload) = frame.into_parts();

                            if let Some(sender) = self.zcl_responses.remove(&header.seq()) {
                                sender.send(payload).unwrap_or_else(|error| {
                                    error!("Failed to send ZCL response: {error:?}");
                                });
                            }
                        }
                    }
                    other => warn!("Unhandled ZCL event: {other:?}"),
                },
            }
        }
    }

    /// Return and increment the ZCL sequence number.
    const fn next_zcl_seq(&mut self) -> u8 {
        let zcl_seq = self.zcl_seq;
        self.zcl_seq = self.zcl_seq.wrapping_add(1);
        zcl_seq
    }

    /// Return and increment the ZDP sequence number.
    const fn next_zdp_seq(&mut self) -> u8 {
        let zdp_seq = self.zdp_seq;
        self.zdp_seq = self.zdp_seq.wrapping_add(1);
        zdp_seq
    }

    /// Send a unicast message.
    async fn unicast(
        &mut self,
        short_id: u16,
        endpoint: Endpoint,
        metadata: Metadata,
        payload: Payload,
    ) -> Result<u8, zigbee_hw::Error> {
        let aps_frame = self.make_aps_frame(metadata, payload);
        self.ncp.unicast(short_id, endpoint, aps_frame).await
    }

    /// Create a new APS frame.
    fn make_aps_frame(&mut self, metadata: Metadata, payload: Payload) -> zigbee_hw::Frame {
        let payload = match payload {
            Payload::Zcl {
                manufacturer_code,
                payload,
            } => self
                .make_zcl_frame(manufacturer_code, *payload)
                .to_le_stream()
                .collect(),
            Payload::Zdp(command) => self.make_zdp_frame(*command).to_le_stream().collect(),
        };

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
        payload: Cluster,
    ) -> zcl::Frame<Cluster> {
        let header = zcl::Header::new(
            payload.scope(),
            payload.direction(),
            payload.disable_default_response(),
            manufacturer_code,
            self.next_zcl_seq(),
            payload.command_id(),
        );

        #[expect(unsafe_code)]
        // SAFETY: We created a valid header above.
        // We trust that the caller has passed in a valid `manufacturer_code`.
        unsafe {
            zcl::Frame::new_unchecked(header, payload)
        }
    }

    /// Create a new ZDP frame.
    const fn make_zdp_frame(&mut self, command: zdp::Command) -> zdp::Frame<zdp::Command> {
        zdp::Frame::new(self.next_zdp_seq(), command)
    }
}
