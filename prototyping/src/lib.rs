//! A library for prototyping Zigbee coordinator devices.

use std::error::Error;
use std::io;

use ezsp::ember::node::Type;
use ezsp::{Networking, Zll};
use log::debug;

/// A Zigbee coordinator device.
pub trait Coordinator {
    /// The error type for coordinator operations.
    type Error: Error;

    /// Initializes the coordinator device.
    ///
    /// # Errors
    ///
    /// Returns an [`io::Error`] if initialization fails.
    fn initialize(&mut self) -> impl Future<Output = io::Result<()>>;

    /// Forms a new Zigbee network with the specified PAN ID and channel.
    ///
    /// # Errors
    ///
    /// Returns an [`io::Error`] if network formation fails.
    fn form_network(
        &mut self,
        pan_id: u16,
        channel: u8,
    ) -> impl Future<Output = Result<(), Self::Error>>;
}

impl<T> Coordinator for T
where
    T: Networking + Zll,
{
    type Error = ezsp::Error;

    async fn initialize(&mut self) -> io::Result<()> {
        debug!("initializing");
        Ok(())
    }

    async fn form_network(&mut self, pan_id: u16, channel: u8) -> Result<(), Self::Error> {
        let (typ, mut params) = self.get_network_parameters().await?;

        debug!("Current node type: {typ}");
        debug!("Current parameters: {params:?}");

        if typ != Type::Coordinator {
            debug!("Setting node type to Coordinator");
            self.set_node_type(Type::Coordinator).await?;
        }

        params.set_pan_id(pan_id);
        params.set_radio_channel(channel);
        debug!("Setting network parameters");
        Networking::form_network(self, params).await?;
        Ok(())
    }
}
