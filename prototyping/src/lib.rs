//! A library for prototyping Zigbee coordinator devices.

use std::error::Error;
use std::io;

use ezsp::ember::node::Type;
use ezsp::ember::zll::{InitialSecurityState, KeyIndex};
use ezsp::{Networking, Zll};
use log::{debug, info};
use rand::random;

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

    /// Permits devices to join the network for a specified duration in seconds.
    ///
    /// # Errors
    ///
    /// Returns an [`io::Error`] if permitting joining fails.
    fn permit_joining(&mut self, seconds: u8) -> impl Future<Output = Result<(), Self::Error>>;
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
        info!("Getting current network parameters");
        let (node_type, mut parameters) = self.get_network_parameters().await?;

        info!("Current node type: {node_type}");
        info!("Current parameters: {parameters:?}");

        if node_type != Type::Coordinator {
            info!("Setting node type to Coordinator");
            self.set_node_type(Type::Coordinator).await?;
        }

        self.set_initial_security_state(
            random(),
            InitialSecurityState::new(
                Default::default(),
                KeyIndex::Development,
                random(),
                random(),
            ),
        )
        .await?;

        parameters.set_pan_id(pan_id);
        parameters.set_radio_channel(channel);

        info!("Setting network parameters");
        Networking::form_network(self, parameters).await
    }

    async fn permit_joining(&mut self, seconds: u8) -> Result<(), Self::Error> {
        info!("Permitting joining for {seconds} seconds");
        self.permit_joining(seconds.into()).await
        // TODO: Send a broadcast to announce the network
    }
}
