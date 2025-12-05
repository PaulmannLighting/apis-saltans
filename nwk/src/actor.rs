use std::collections::BTreeMap;
use std::time::Duration;

use log::error;
use macaddr::MacAddr8;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::oneshot;
use zigbee::Endpoint;

use crate::device_proxy::DeviceProxy;
pub use crate::message::Message;
use crate::zcl_proxy::ZclProxy;
use crate::{Error, Nlme};

/// Actor trait for handling NWK layer messages.
pub trait Actor {
    /// Run the actor, processing incoming messages.
    fn run(self, rx: Receiver<Message>) -> impl Future<Output = ()>;
}

impl<T> Actor for T
where
    T: Nlme,
{
    async fn run(mut self, mut rx: Receiver<Message>) {
        while let Some(message) = rx.recv().await {
            match message {
                Message::GetTransactionSeq { response } => {
                    response
                        .send(self.get_transaction_seq())
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
                Message::Unicast {
                    pan_id,
                    endpoint,
                    cluster_id,
                    payload,
                    response,
                } => {
                    response
                        .send(self.unicast(pan_id, endpoint, cluster_id, payload).await)
                        .unwrap_or_else(|error| {
                            error!("Failed to send ZCL command response: {error:?}");
                        });
                }
            }
        }
    }
}
