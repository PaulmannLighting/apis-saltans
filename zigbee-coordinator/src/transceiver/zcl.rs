//! Transceiver to send and receive ZCL messages.

use std::collections::BTreeMap;

use le_stream::ToLeStream;
use log::{error, warn};
use tokio::sync::mpsc::Receiver;
use tokio::sync::oneshot;
use tokio::sync::oneshot::{Sender, channel};
use zcl::{Cluster, CommandDispatch};
use zigbee::{Address, Endpoint};
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
                    payload,
                    response,
                } => {
                    self.unicast(address, endpoint, *payload, response).await;
                }
                Message::Communicate {
                    address,
                    endpoint,
                    payload,
                    response,
                } => {
                    self.communicate(address, endpoint, *payload, response)
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
        address: Address,
        endpoint: Endpoint,
        payload: Payload,
        response: Sender<Result<(), zigbee_hw::Error>>,
    ) {
        let (metadata, manufacturer_code, command) = payload.into_parts();
        let zcl_frame = self.make_zcl_frame(manufacturer_code, command);
        let aps_frame = Self::make_aps_frame(metadata, zcl_frame);
        let result = self.ncp.unicast(address, endpoint, aps_frame).await;
        response.send(result.map(drop)).unwrap_or_else(|error| {
            error!("Failed to send unicast response: {error:?}");
        });
    }

    /// Send a unicast message with back-channel communication.
    async fn communicate(
        &mut self,
        address: Address,
        endpoint: Endpoint,
        payload: Payload,
        response: Sender<Result<oneshot::Receiver<Cluster>, zigbee_hw::Error>>,
    ) {
        let (metadata, manufacturer_code, command) = payload.into_parts();
        let zcl_frame = self.make_zcl_frame(manufacturer_code, command);
        let seq = zcl_frame.header().seq();
        let aps_frame = Self::make_aps_frame(metadata, zcl_frame);

        match self.ncp.unicast(address, endpoint, aps_frame).await {
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
    fn make_aps_frame(metadata: Metadata, frame: zcl::Frame<Cluster>) -> zigbee_hw::Frame {
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
        // SAFETY: We created a valid header above.
        // We trust that the caller has passed in a valid `manufacturer_code`.
        unsafe {
            zcl::Frame::new_unchecked(header, command)
        }
    }
}
