//! A library for prototyping Zigbee coordinator devices.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::time::Duration;

use ezsp::ember::security::initial;
use ezsp::ember::{aps, child, concentrator, network, node};
use ezsp::ezsp::{config, decision, policy};
use ezsp::{Configuration, Messaging, Networking, Security, Utilities, Zll};
use le_stream::ToLeStream;
use log::{debug, error, info};
use macaddr::MacAddr8;
use zdp::MgmtPermitJoiningReq;

const LINK_KEY: &[u8] = include_bytes!("../../assets/link.key");

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

    /// Sets the policy for the coordinator.
    ///
    /// # Errors
    ///
    /// Returns an [`Self::Error`] if setting the configuration policy fails.
    fn set_stack_policy(
        &mut self,
        policy: BTreeMap<policy::Id, decision::Id>,
    ) -> impl Future<Output = Result<(), Self::Error>>;

    /// Sets the stack configuration for the coordinator.
    ///
    /// # Errors
    ///
    /// Returns an [`Self::Error`] if setting the stack configuration fails.
    fn set_stack_configuration(
        &mut self,
        configuration: BTreeMap<config::Id, u16>,
    ) -> impl Future<Output = Result<(), Self::Error>>;

    /// Starts up the coordinator device, optionally reinitializing it.
    ///
    /// # Errors
    ///
    /// Returns an [`Self::Error`] if startup fails.
    fn startup(&mut self, reinitialize: bool) -> impl Future<Output = Result<(), Self::Error>>;

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
    T: Configuration + Messaging + Networking + Security + Utilities + Zll,
{
    type Error = ezsp::Error;

    async fn initialize(&mut self) -> Result<(), Self::Error> {
        debug!("initializing");

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

        let mut configuration = BTreeMap::new();
        configuration.insert(config::Id::SourceRouteTableSize, 100);
        configuration.insert(config::Id::ApsUnicastMessageCount, 16);
        configuration.insert(config::Id::NeighborTableSize, 24);
        configuration.insert(config::Id::MaxHops, 30);
        self.set_stack_configuration(configuration).await?;

        let mut policy = BTreeMap::new();
        policy.insert(policy::Id::TrustCenter, decision::Id::AllowJoins);
        self.set_stack_policy(policy).await?;

        info!("Getting current network parameters");
        let (node_type, parameters) = self.get_network_parameters().await?;
        info!("Node type: {node_type:?}");
        log_parameters(&parameters);

        let ieee_address = self.get_eui64().await?;
        info!("IEEE address: {ieee_address}");

        self.set_radio_power(8).await?;

        let link_key = LINK_KEY
            .try_into()
            .expect("Link key should be valid. This is a bug.");
        info!("Link key: {link_key:02X?}");

        // Randomly generated network key for testing.
        let network_key = [
            0xCB, 0x61, 0x6A, 0x55, 0xA2, 0xA6, 0x9D, 0x7D, 0x7F, 0x71, 0x4F, 0xAD, 0x88, 0xA5,
            0xD4, 0x9E,
        ];
        info!("Network key: {network_key:02X?}");
        Security::set_initial_security_state(
            self,
            initial::State::new(
                [
                    initial::Bitmask::HavePreconfiguredKey,
                    initial::Bitmask::RequireEncryptedKey,
                ]
                .into(),
                link_key,
                network_key,
                0,
                MacAddr8::default(),
            ),
        )
        .await?;

        Ok(())
    }

    async fn set_stack_policy(
        &mut self,
        policy: BTreeMap<policy::Id, decision::Id>,
    ) -> Result<(), Self::Error> {
        for (key, value) in policy {
            info!("Setting policy {key:?} to {value:?}");
            self.set_policy(key, value).await?;
        }

        Ok(())
    }

    async fn set_stack_configuration(
        &mut self,
        configuration: BTreeMap<config::Id, u16>,
    ) -> Result<(), Self::Error> {
        for (key, value) in configuration {
            info!("Setting configuration {key:?} to {value}");
            self.set_configuration_value(key, value).await?;
        }

        Ok(())
    }

    async fn startup(&mut self, reinitialize: bool) -> Result<(), Self::Error> {
        let input_cluster_list = [0x0000, 0x0006, 0x0008, 0x0300, 0x0403, 0x0201];
        let output_cluster_list = [0x0000, 0x0006, 0x0008, 0x0300, 0x0403];
        info!("Adding endpoint");
        self.add_endpoint(
            1,
            0x0104,
            0x0050,
            0,
            input_cluster_list.into_iter().collect(),
            output_cluster_list.into_iter().collect(),
        )
        .await?;

        info!("Initializing network");
        if let Err(error) = self.network_init(BTreeSet::default()).await {
            error!("Failed to initialize network: {error}");
        }

        info!("Getting security state");
        match self.get_current_security_state().await {
            Ok(state) => {
                info!("Current security state: {state:?}");
            }
            Err(error) => {
                error!("Failed to get security state: {error}");
            }
        }

        info!("Getting network state");
        match self.network_state().await {
            Ok(state) => {
                info!("Current network state: {state:?}");
            }
            Err(error) => {
                error!("Failed to get network state: {error}");
            }
        }

        info!("Getting network parameters - pre reinitialization");
        let (node_type, mut parameters) = self.get_network_parameters().await?;
        info!("Node type: {node_type:?}");
        log_parameters(&parameters);

        if reinitialize {
            info!("Leaving network");
            if let Err(error) = self.leave_network().await {
                error!("Failed to leave network: {error}");
            }

            let pan_id: u16 = 24171;
            info!("Pan id: {pan_id}");
            parameters.set_pan_id(pan_id);

            let extended_pan_id = MacAddr8::new(0x8D, 0x9F, 0x3D, 0xFE, 0x00, 0xBF, 0x0D, 0xB5);
            info!("Extended pan id: {extended_pan_id}");
            parameters.set_extended_pan_id(extended_pan_id);

            parameters.set_radio_channel(12);

            if node_type == node::Type::Coordinator {
                info!("Forming network");
                Networking::form_network(self, parameters).await?;
            } else {
                self.join_network(node_type, parameters).await?;
            }
        } else if node_type == node::Type::Router {
            info!("Rejoining network");
            self.find_and_rejoin_network(true, 0).await?;
        }

        info!("Getting network parameters - post reinitialization");
        let (node_type, parameters) = self.get_network_parameters().await?;
        info!("Node type: {node_type:?}");
        log_parameters(&parameters);

        // TODO: Only if concentrator type is set.
        info!("Sending many-to-one route request");
        self.send_many_to_one_route_request(concentrator::Type::HighRam, 8)
            .await?;

        Ok(())
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
            }
        }

        nodes
    }
}

fn log_parameters(parameters: &network::Parameters) {
    info!("PAN ID: {:#X}", parameters.pan_id());
    info!("Extended PAN ID: {:#X?}", parameters.extended_pan_id());
    info!("Radio TX power: {:#X}", parameters.radio_tx_power());
    info!("Radio channel: {:#X}", parameters.radio_channel());
    info!("Join method: {:#X?}", parameters.join_method());
    info!("Nwk manager ID: {:#X}", parameters.nwk_manager_id());
    info!("Nwk update ID: {:#X}", parameters.nwk_update_id());
    info!("Channels: {:#X}", parameters.channels());
}
