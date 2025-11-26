//! A library for prototyping Zigbee coordinator devices.

use std::collections::BTreeMap;
use std::error::Error;
use std::time::Duration;

use ezsp::ember::node::Type;
use ezsp::ember::zll::{InitialSecurityState, KeyIndex};
use ezsp::ember::{aps, child, concentrator};
use ezsp::ezsp::{config, decision, policy};
use ezsp::{Configuration, Messaging, Networking, Zll};
use le_stream::ToLeStream;
use log::{debug, info};
use rand::random;
use zdp::MgmtPermitJoiningReq;

/// A Zigbee coordinator device.
pub trait Coordinator {
    /// The error type for coordinator operations.
    type Error: Error;

    /// Initializes the coordinator device.
    ///
    /// # Errors
    ///
    /// Returns an [`Self::Error`] if initialization fails.
    fn initialize(&mut self) -> impl Future<Output = Result<(), Self::Error>>;

    /// Forms a new Zigbee network with the specified PAN ID and channel.
    ///
    /// # Errors
    ///
    /// Returns an [`Self::Error`] if network formation fails.
    fn form_network(
        &mut self,
        pan_id: u16,
        channel: u8,
    ) -> impl Future<Output = Result<(), Self::Error>>;

    /// Permits devices to join the network for a specified duration in seconds.
    ///
    /// # Errors
    ///
    /// Returns an [`Self::Error`] if permitting joining fails.
    fn permit_joining(&mut self, seconds: u8) -> impl Future<Output = Result<(), Self::Error>>;

    /// Advertises the network to nearby devices for the specified duration.
    ///
    /// # Errors
    ///
    /// Returns an [`Self::Error`] if advertising the network fails.
    fn advertise_network(&mut self, seconds: u8) -> impl Future<Output = Result<(), Self::Error>>;

    /// Retrieves a list of nodes currently connected to the network.
    ///
    /// # Errors
    ///
    /// Returns an [`Self::Error`] if retrieving the nodes fails.
    fn get_children(&mut self) -> impl Future<Output = BTreeMap<u8, child::Data>>;
}

impl<T> Coordinator for T
where
    T: Configuration + Messaging + Networking + Zll,
{
    type Error = ezsp::Error;

    async fn initialize(&mut self) -> Result<(), Self::Error> {
        debug!("initializing");

        // See `InitZigBeeLibraryService.java`.
        self.set_policy(policy::Id::TrustCenter, decision::Id::AllowJoins)
            .await?;
        self.set_radio_power(8).await?;
        let config = concentrator::Parameters::new(
            concentrator::Type::HighRam,
            Duration::from_secs(60),
            Duration::from_secs(3600),
            8,
            8,
            0,
        )
        .expect("Concentrator parameters should be valid.");
        self.set_concentrator(Some(config)).await?;
        self.set_configuration_value(config::Id::SourceRouteTableSize, 100)
            .await?;
        self.set_configuration_value(config::Id::ApsUnicastMessageCount, 16)
            .await?;
        self.set_configuration_value(config::Id::NeighborTableSize, 24)
            .await?;
        self.set_configuration_value(config::Id::MaxHops, 30)
            .await?;
        Ok(())
    }

    async fn form_network(&mut self, pan_id: u16, channel: u8) -> Result<(), Self::Error> {
        info!("Getting current network parameters");
        let parameters = self.get_network_parameters().await?;

        let status = parameters.status();
        let node_type = parameters.node_type();
        let mut parameters = parameters.into_parameters();

        info!("Current status: {status:?}");
        info!("Current node type: {node_type:?}");
        info!("Current parameters: {parameters:?}");

        if node_type != Ok(Type::Coordinator) {
            info!("Setting node type to Coordinator");
            self.set_node_type(Type::Coordinator).await?;
        }

        self.set_initial_security_state(
            random(),
            InitialSecurityState::new(
                Default::default(),
                KeyIndex::Certification,
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
        self.permit_joining(seconds.into()).await?;
        Ok(())
    }

    async fn advertise_network(&mut self, seconds: u8) -> Result<(), Self::Error> {
        let mut options = aps::Options::new();
        options
            .push(aps::Option::Retry)
            .expect("Options buffer should have sufficient capacity. This is a bug.");
        options
            .push(aps::Option::EnableAddressDiscovery)
            .expect("Options buffer should have sufficient capacity. This is a bug.");
        options
            .push(aps::Option::EnableRouteDiscovery)
            .expect("Options buffer should have sufficient capacity. This is a bug.");
        let message = zdp::Frame::new(0x00, MgmtPermitJoiningReq::new(seconds, true));
        info!("ZDP frame: {message:x?}");
        let aps_frame = aps::Frame::new(0, message.cluster_id(), 0, 0, options, 0, 1);
        info!("APS Frame: {aps_frame:x?}");

        info!("Sending broadcast to notify devices");
        self.send_broadcast(
            0xFFFC,
            aps_frame.clone(),
            0x08,
            0x26,
            message.clone().to_le_stream().collect(),
        )
        .await?;

        Ok(())
    }

    async fn get_children(&mut self) -> BTreeMap<u8, child::Data> {
        let mut nodes = BTreeMap::new();

        for index in 0..=u8::MAX {
            if let Ok(child) = self.get_child_data(index).await {
                info!("Child at {index}: {child:?}");
                nodes.insert(index, child);
            } else {
                debug!("No child at index {index}");
            }
        }

        nodes
    }
}
