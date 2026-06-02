use log::{error, trace};
use tokio::spawn;
use tokio::sync::mpsc::{Receiver, channel};
use tokio::task::JoinHandle;

use crate::message::Message;
use crate::{Ncp, NcpDriver};

/// Sealed driver trait for handling communication with the Zigbee NCP.
///
/// This trait should not be implemented directly. Instead, implement the `NcpDriver` trait
/// for your  NCP type, and this `SealedDriver` trait will be automatically implemented for it.
pub trait SealedDriver {
    /// Run the actor, processing incoming messages.
    fn run(self, rx: Receiver<Message>) -> impl Future<Output = ()> + Send;

    /// Spawn the actor in a new tokio task.
    ///
    /// # Returns
    ///
    /// Returns a tuple of the tokio task's join handle and an actor proxy.
    fn spawn(self, channel_size: usize) -> (JoinHandle<()>, impl Ncp + Clone + Send)
    where
        Self: 'static;
}

impl<T> SealedDriver for T
where
    T: NcpDriver + Send + 'static,
{
    #[expect(clippy::too_many_lines)]
    async fn run(mut self, mut rx: Receiver<Message>) {
        while let Some(message) = rx.recv().await {
            match message {
                Message::GetTransactionSeq { response } => {
                    response
                        .send(self.next_transaction_seq())
                        .unwrap_or_else(|error| {
                            error!("Failed to send get PAN ID command response: {error:?}");
                        });
                }
                Message::GetPanId { response } => {
                    response
                        .send(self.get_short_id().await)
                        .unwrap_or_else(|error| {
                            error!("Failed to send get PAN ID command response: {error:?}");
                        });
                }
                Message::ScanNetworks {
                    channel_mask,
                    duration,
                    response,
                } => {
                    response
                        .send(self.scan_networks(channel_mask, duration).await)
                        .unwrap_or_else(|error| {
                            error!("Failed to send scan networks command response: {error:?}");
                        });
                }
                Message::ScanChannels {
                    channel_mask,
                    duration,
                    response,
                } => {
                    response
                        .send(self.scan_channels(channel_mask, duration).await)
                        .unwrap_or_else(|error| {
                            error!("Failed to send scan channels command response: {error:?}");
                        });
                }
                Message::AllowJoins { duration, response } => {
                    response
                        .send(self.allow_joins(duration).await)
                        .unwrap_or_else(|error| {
                            error!("Failed to send allow joins command response: {error:?}");
                        });
                }
                Message::GetNeighbors { response } => {
                    response
                        .send(self.get_neighbors().await)
                        .unwrap_or_else(|error| {
                            error!("Failed to send get neighbors command response: {error:?}");
                        });
                }
                Message::RouteRequest { radius, response } => {
                    response
                        .send(self.route_request(radius).await)
                        .unwrap_or_else(|error| {
                            error!("Failed to send route request command response: {error:?}");
                        });
                }
                Message::GetIeeeAddress { short_id, response } => {
                    response
                        .send(self.get_ieee_address(short_id).await)
                        .unwrap_or_else(|error| {
                            error!("Failed to send get IEEE address command response: {error:?}");
                        });
                }
                Message::Unicast {
                    short_id,
                    endpoint,
                    frame,
                    response,
                } => {
                    response
                        .send(self.unicast(short_id, endpoint, frame).await)
                        .unwrap_or_else(|error| {
                            error!("Failed to send ZCL command response: {error:?}");
                        });
                }
                Message::Multicast {
                    group_id,
                    hops,
                    radius,
                    frame,
                    response,
                } => {
                    response
                        .send(self.multicast(group_id, hops, radius, frame).await)
                        .unwrap_or_else(|error| {
                            error!("Failed to send multicast command response: {error:?}");
                        });
                }
                Message::Broadcast {
                    short_id,
                    radius,
                    frame,
                    response,
                } => {
                    response
                        .send(self.broadcast(short_id, radius, frame).await)
                        .unwrap_or_else(|error| {
                            error!("Failed to send broadcast command response: {error:?}");
                        });
                }
                Message::Subscribe { sender } => {
                    self.subscribe(sender).await;
                }
            }
        }

        trace!("Message channel closed, NWK actor exiting.");
    }

    fn spawn(self, channel_size: usize) -> (JoinHandle<()>, impl Ncp + Clone + Send)
    where
        Self: 'static,
    {
        let (tx, rx) = channel(channel_size);
        let join_handle = spawn(self.run(rx));
        (join_handle, tx)
    }
}
