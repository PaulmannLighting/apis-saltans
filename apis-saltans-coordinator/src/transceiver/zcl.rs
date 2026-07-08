//! Transceiver to send and receive ZCL messages.

use std::collections::BTreeMap;

use apis_saltans_aps::Data;
use apis_saltans_core::Application;
use apis_saltans_hw::Ncp;
use apis_saltans_nwk::Source;
use apis_saltans_zcl::{Cluster, Frame};
use le_stream::ToLeStream;
use log::{debug, trace, warn};
use tokio::spawn;
use tokio::sync::mpsc::{Receiver, WeakSender};
use tokio::sync::oneshot::{self, Sender, channel};

pub use self::handle::Handle;
pub use self::message::{Message, Payload};
use super::index::Index;
use crate::{MPSC_CHANNEL_SIZE, network_manager};

mod handle;
mod message;

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
                Message::Unicast {
                    short_id,
                    endpoint,
                    payload,
                    response,
                } => {
                    let seq = self.next_seq();
                    response
                        .send(
                            self.unicast(seq, short_id, endpoint, payload)
                                .await
                                .map(drop),
                        )
                        .unwrap_or_else(|error| {
                            debug!("Failed to send unicast response: {error:?}");
                        });
                }
                Message::Multicast {
                    group_id,
                    hops,
                    radius,
                    payload,
                    response,
                } => {
                    response
                        .send(
                            self.multicast(group_id, hops, radius, payload)
                                .await
                                .map(drop),
                        )
                        .unwrap_or_else(|error| {
                            debug!("Failed to send unicast response: {error:?}");
                        });
                }
                Message::Communicate {
                    short_id,
                    endpoint,
                    payload,
                    response,
                } => {
                    response
                        .send(self.communicate(short_id, endpoint, payload).await)
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
    async fn unicast(
        &self,
        seq: u8,
        short_id: u16,
        endpoint: Application,
        frame: Payload<Cluster>,
    ) -> Result<u8, apis_saltans_hw::Error> {
        let (metadata, manufacturer_code, command) = frame.into_parts();
        let zcl_frame = Frame::new(seq, manufacturer_code, command);
        let payload = zcl_frame.to_le_stream().collect();
        let hw_frame = apis_saltans_hw::Frame::new(metadata, payload);

        self.ncp
            .unicast(short_id, endpoint.into(), hw_frame)
            .await
            .map(|_| seq)
    }

    /// Send a ZCL multicast message.
    ///
    /// # Returns
    ///
    /// Returns the ZCL sequence number.
    ///
    /// # Errors
    ///
    /// Returns an error if the message could not be sent.
    async fn multicast(
        &mut self,
        group_id: u16,
        hops: u8,
        radius: u8,
        frame: Payload<Cluster>,
    ) -> Result<u8, apis_saltans_hw::Error> {
        let (metadata, manufacturer_code, command) = frame.into_parts();
        let seq = self.next_seq();
        let zcl_frame = Frame::new(seq, manufacturer_code, command);
        let payload = zcl_frame.to_le_stream().collect();
        let hw_frame = apis_saltans_hw::Frame::new(metadata, payload);

        self.ncp
            .multicast(group_id, hops, radius, hw_frame)
            .await
            .map(|_| seq)
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
        short_id: u16,
        endpoint: Application,
        frame: Payload<Cluster>,
    ) -> Result<oneshot::Receiver<Cluster>, apis_saltans_hw::Error> {
        let seq = self.next_seq();
        let index = Index::from_sent_payload(short_id, endpoint, seq, &frame);
        self.unicast(seq, short_id, endpoint, frame).await?;
        let (tx, rx) = channel();
        self.responses.insert(index, tx);
        Ok(rx)
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
