use log::{error, trace};
use tokio::sync::mpsc::Receiver;

use crate::NetworkManager;
use crate::message::Message;

/// Sealed actor trait for handling communication with the Zigbee NCP.
///
/// This trait should not be implemented directly. Instead, implement the `NetworkManager` trait
/// for your  NCP type, and the `Actor` trait will be automatically implemented for it.
pub trait Actor {
    /// Run the actor, processing incoming messages.
    fn run(self, rx: Receiver<Message>) -> impl Future<Output = ()>;
}

impl<T> Actor for T
where
    T: NetworkManager,
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
                        .send(self.get_pan_id().await)
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
                Message::GetIeeeAddress { pan_id, response } => {
                    response
                        .send(self.get_ieee_address(pan_id).await)
                        .unwrap_or_else(|error| {
                            error!("Failed to send get IEEE address command response: {error:?}");
                        });
                }
                Message::Unicast {
                    pan_id,
                    endpoint,
                    frame,
                    response,
                } => {
                    response
                        .send(self.unicast(pan_id, endpoint, frame).await)
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
                    pan_id,
                    radius,
                    frame,
                    response,
                } => {
                    response
                        .send(self.broadcast(pan_id, radius, frame).await)
                        .unwrap_or_else(|error| {
                            error!("Failed to send broadcast command response: {error:?}");
                        });
                }
            }
        }

        trace!("Message channel closed, NWK actor exiting.");
    }
}
