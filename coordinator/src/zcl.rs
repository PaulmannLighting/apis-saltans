//! Transceiver to send and receive ZCL messages.

use std::collections::BTreeMap;

use bytes::Bytes;
use le_stream::ToLeStream;
use log::{debug, trace, warn};
use tokio::spawn;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::oneshot::{self, channel};
use zb_aps::Data;
use zb_core::Destination;
use zb_core::destination::Device;
use zb_nwk::Source;
use zb_zcl::{Cluster, Frame, Header};

pub use self::message::Message;
pub use self::payload::{Metadata, Payload};
use super::index::Index;
use crate::aps::Aps;
use crate::response::InternalCommunicationResponse;
use crate::{Event, MPSC_CHANNEL_SIZE};

mod message;
mod payload;

/// Zigbee transceiver actor.
#[derive(Debug)]
pub struct Transceiver {
    aps: Aps,
    events: Sender<Event>,
    responses: BTreeMap<Index, oneshot::Sender<Cluster>>,
    seq: u8,
}

impl Transceiver {
    /// Create a new transceiver.
    #[must_use]
    pub const fn new(aps: Aps, events: Sender<Event>) -> Self {
        Self {
            aps,
            events,
            responses: BTreeMap::new(),
            seq: 0,
        }
    }
    /// Run the transceiver.
    pub async fn run(mut self, mut messages: Receiver<Message>) {
        while let Some(message) = messages.recv().await {
            match message {
                Message::Received { source, frame } => {
                    self.handle_message_received(source, frame).await;
                }
                Message::Transmit {
                    destination,
                    payload,
                    response,
                } => {
                    response
                        .send(self.transmit(destination, payload).await)
                        .unwrap_or_else(|error| {
                            debug!("Failed to send unicast response: {error:?}");
                        });
                }
                Message::Communicate {
                    device,
                    payload,
                    response,
                } => {
                    response
                        .send(self.communicate(device, payload).await)
                        .unwrap_or_else(|error| {
                            debug!("Failed to send unicast response: {error:?}");
                        });
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

    /// Handle a received ZCL message.
    async fn handle_message_received(&mut self, source: Source, aps_frame: Data<Frame<Cluster>>) {
        trace!("Received ZCL message from {source}: {aps_frame:?}");
        let index = Index::from_received_zcl_frame(source, &aps_frame);

        if let Some(sender) = self.responses.remove(&index) {
            let (_, zcl_frame) = aps_frame.into_parts();
            let (_, cluster) = zcl_frame.into_parts();
            sender.send(cluster).unwrap_or_else(|error| {
                debug!("Failed to send ZCL response: {error:?}");
            });

            return;
        }

        let Ok(short_id) = source.node_id().try_into().inspect_err(|error| {
            warn!("Discarding message from invalid source: {source}: {error:?}");
        }) else {
            return;
        };

        self.events
            .send(Event::Zcl {
                src_address: short_id,
                aps_frame,
            })
            .await
            .unwrap_or_else(|error| {
                debug!("Failed to send command: {error:?}");
            });
    }

    /// Send a ZCL unicast message.
    ///
    /// # Returns
    ///
    /// Returns the ZCL sequence number.
    ///
    /// # Errors
    ///
    /// Returns an error if the unicast message could not be sent.
    async fn transmit(
        &mut self,
        destination: Destination,
        payload: Payload,
    ) -> Result<(), zb_hw::Error> {
        let (aps_metadata, zcl_metadata, command) = payload.into_parts();
        let zcl_frame = self.make_zcl_frame(zcl_metadata, command);
        self.aps
            .transmit(
                destination,
                aps_metadata,
                zcl_frame.to_le_stream().collect(),
            )
            .await
    }

    /// Send a ZCL unicast message with back-channel communication.
    ///
    /// # Returns
    ///
    /// Returns the response receiver.
    ///
    /// # Errors
    ///
    /// Returns an error if the unicast message could not be sent.
    async fn communicate(
        &mut self,
        device: Device,
        datagram: Payload,
    ) -> Result<InternalCommunicationResponse<Cluster>, zb_hw::Error> {
        let (aps_metadata, zcl_metadata, command) = datagram.into_parts();
        let zcl_frame = self.make_zcl_frame(zcl_metadata, command);
        let index = Index::from_zcl_command(
            device,
            zcl_frame.header().seq(),
            aps_metadata,
            zcl_metadata.manufacturer_code,
        );
        let destination = Destination::from(device);
        let payload = zcl_frame.to_le_stream().collect();
        let (tx, rx) = channel();
        self.responses.insert(index, tx);

        if let Err(error) = self.aps.transmit(destination, aps_metadata, payload).await {
            self.responses.remove(&index);
            return Err(error);
        }

        Ok(InternalCommunicationResponse::new(rx))
    }

    fn make_zcl_frame(&mut self, metadata: Metadata, command: Bytes) -> Frame<Bytes> {
        let header = Header::new(
            metadata.scope,
            metadata.direction,
            metadata.disable_default_response,
            metadata.manufacturer_code,
            self.next_seq(),
            metadata.command_id,
        );
        #[expect(unsafe_code)]
        // SAFETY: We safely construct the frame from the correct metadata
        // with a freshly incremented sequence number.
        unsafe {
            Frame::new_unchecked(header, command)
        }
    }
}

impl Transceiver {
    /// Start the ZCL transceiver.
    pub fn spawn(aps: Aps, events: Sender<Event>) -> Sender<Message> {
        let (zcl_tx, zcl_rx) = tokio::sync::mpsc::channel(MPSC_CHANNEL_SIZE);
        spawn(Self::new(aps, events).run(zcl_rx));
        zcl_tx
    }
}
