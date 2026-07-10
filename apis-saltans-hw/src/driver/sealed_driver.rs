use tokio::sync::mpsc::{Receiver, channel};

use super::Driver;
use crate::common::NcpHandle;
use crate::common::message::Message;

/// Sealed driver trait for handling actor communication with the Zigbee NCP.
///
/// This trait should not be implemented directly. Instead, implement the [`Driver`] trait for your
/// NCP type, and this `SealedDriver` trait will be automatically implemented for it.
pub trait SealedDriver {
    /// Run the actor, processing incoming messages.
    fn run(self, rx: Receiver<Message>) -> impl Future<Output = Self> + Send;

    /// Spawn the actor in a new tokio task.
    ///
    /// # Returns
    ///
    /// Returns a tuple of the tokio task's join handle and an actor proxy.
    fn spawn(self, channel_size: usize) -> (NcpHandle, impl Future<Output = Self> + Send)
    where
        Self: Sized + 'static;
}

impl<T> SealedDriver for T
where
    T: Driver + Send + 'static,
{
    async fn run(mut self, mut rx: Receiver<Message>) -> Self {
        while let Some(message) = rx.recv().await {
            match message {
                Message::GetPanId { response } => {
                    response.send(self.get_pan_id().await).unwrap_or_else(drop);
                }
                Message::GetIeeeAddress { response } => {
                    response
                        .send(self.get_ieee_address().await)
                        .unwrap_or_else(drop);
                }
                Message::ScanNetworks {
                    channel_mask,
                    duration,
                    response,
                } => {
                    response
                        .send(self.scan_networks(channel_mask, duration).await)
                        .unwrap_or_else(drop);
                }
                Message::ScanChannels {
                    channel_mask,
                    duration,
                    response,
                } => {
                    response
                        .send(self.scan_channels(channel_mask, duration).await)
                        .unwrap_or_else(drop);
                }
                Message::AllowJoins { duration, response } => {
                    response
                        .send(self.allow_joins(duration).await)
                        .unwrap_or_else(drop);
                }
                Message::RouteRequest { radius, response } => {
                    response
                        .send(self.route_request(radius).await)
                        .unwrap_or_else(drop);
                }
                Message::TranslateIeeeAddress { short_id, response } => {
                    response
                        .send(self.short_id_to_ieee_address(short_id).await)
                        .unwrap_or_else(drop);
                }
                Message::TranslateShortId {
                    ieee_address,
                    response,
                } => {
                    response
                        .send(self.ieee_address_to_short_id(ieee_address).await)
                        .unwrap_or_else(drop);
                }
                Message::Transmit {
                    destination,
                    datagram,
                    response,
                } => {
                    response
                        .send(self.transmit(destination, datagram).await)
                        .unwrap_or_else(drop);
                }
            }
        }

        self
    }

    fn spawn(self, channel_size: usize) -> (NcpHandle, impl Future<Output = Self> + Send)
    where
        Self: 'static,
    {
        let (tx, rx) = channel(channel_size);
        let future = self.run(rx);
        (tx, future)
    }
}
