//! Transceiver to send and receive ZCL messages.

use std::collections::BTreeMap;

use apis_saltans_aps::Data;
use apis_saltans_core::Destination;
use apis_saltans_core::destination::Device;
use apis_saltans_hw::Ncp;
use apis_saltans_nwk::Source;
use apis_saltans_zcl::{Cluster, Frame, Header};
use bytes::Bytes;
use le_stream::ToLeStream;
use log::{debug, trace, warn};
use tokio::spawn;
use tokio::sync::mpsc::{Receiver, WeakSender};
use tokio::sync::oneshot::{self, Sender, channel};

pub use self::handle::Handle;
pub use self::message::Message;
pub use self::payload::{Metadata, Payload};
use super::index::Index;
use crate::{MPSC_CHANNEL_SIZE, network_manager};

mod handle;
mod message;
mod payload;

/// Zigbee transceiver actor.
#[derive(Debug)]
pub struct Transceiver<T> {
    ncp: T,
    network_manager: WeakSender<network_manager::Message>,
    responses: BTreeMap<Index, Sender<Cluster>>,
    seq: u8,
}

impl<T> Transceiver<T> {
    /// Create a new transceiver.
    #[must_use]
    pub const fn new(ncp: T, network_manager: WeakSender<network_manager::Message>) -> Self {
        Self {
            ncp,
            network_manager,
            responses: BTreeMap::new(),
            seq: 0,
        }
    }
}

impl<T> Transceiver<T>
where
    T: Ncp + Sync,
{
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
                        .send(self.transmit(destination, payload).await.map(drop))
                        .unwrap_or_else(|error| {
                            debug!("Failed to send unicast response: {error:?}");
                        });
                }
                Message::Communicate {
                    destination,
                    payload,
                    response,
                } => {
                    response
                        .send(self.communicate(destination, payload).await)
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

        let Some(sender) = self.network_manager.upgrade() else {
            warn!("Network manager actor has been dropped");
            return;
        };

        sender
            .send(network_manager::Message::Command {
                source,
                frame: aps_frame,
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
    ) -> Result<u8, apis_saltans_hw::Error> {
        let (aps_metadata, zcl_metadata, command) = payload.into_parts();
        let zcl_frame = self.make_zcl_frame(zcl_metadata, command);
        let zcl_seq = zcl_frame.header().seq();
        let hw_datagram = make_hw_datagram(aps_metadata, zcl_frame);
        self.ncp.transmit(destination, hw_datagram).await?;
        Ok(zcl_seq)
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
        destination: Device,
        datagram: Payload,
    ) -> Result<oneshot::Receiver<Cluster>, apis_saltans_hw::Error> {
        let (aps_metadata, zcl_metadata, command) = datagram.into_parts();
        let zcl_frame = self.make_zcl_frame(zcl_metadata, command);
        let index = Index::from_zcl_command(
            destination,
            zcl_frame.header().seq(),
            aps_metadata,
            zcl_metadata.manufacturer_code,
        );
        let hw_datagram = make_hw_datagram(aps_metadata, zcl_frame);
        let (tx, rx) = channel();
        self.responses.insert(index, tx);
        self.ncp.transmit(destination.into(), hw_datagram).await?;
        Ok(rx)
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

impl<T> Transceiver<T>
where
    T: Ncp + Send + Sync + 'static,
{
    /// Start the ZCL transceiver.
    pub fn spawn(
        ncp: T,
        network_manager: WeakSender<network_manager::Message>,
    ) -> tokio::sync::mpsc::Sender<Message> {
        let (zcl_tx, zcl_rx) = tokio::sync::mpsc::channel(MPSC_CHANNEL_SIZE);
        spawn(Self::new(ncp, network_manager).run(zcl_rx));
        zcl_tx
    }
}

fn make_hw_datagram(
    metadata: apis_saltans_hw::Metadata,
    payload: Frame<Bytes>,
) -> apis_saltans_hw::Datagram {
    #[expect(unsafe_code)]
    // SAFETY: We safely construct the datagram from the correct metadata we destructured above.
    unsafe {
        apis_saltans_hw::Datagram::new_unchecked(metadata, payload.to_le_stream().collect())
    }
}
