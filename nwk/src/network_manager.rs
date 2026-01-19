use log::{debug, error, info, warn};
use macaddr::MacAddr8;
use tokio::sync::mpsc::{Receiver, Sender, channel};
use zcl::Cluster;
use zcl::general::on_off;
use zdp::{BindReq, Destination};
use zigbee::Endpoint;
use zigbee::node::{
    Descriptor, Flags, FrequencyBand, LogicalType, MacCapabilityFlags, Node, ServerMask,
};

use self::network_state::NetworkState;
use crate::{Command, Error, Event, Proxy};

mod network_state;

/// Zigbee Network Manager handling communication via an NCP actor.
#[derive(Debug)]
pub struct NetworkManager<T> {
    proxy: T,
    events_in: Receiver<Event>,
    events_out: Sender<Event>,
    state: NetworkState,
}

impl<T> NetworkManager<T> {
    /// Creates a new `NetworkManager`.
    #[must_use]
    pub fn new(
        proxy: T,
        events_in: Receiver<Event>,
        channel_size: usize,
    ) -> (Self, Receiver<Event>) {
        let (events_out, events_rx) = channel(channel_size);
        (
            Self {
                proxy,
                events_in,
                events_out,
                state: NetworkState::new(),
            },
            events_rx,
        )
    }
}

impl<T> NetworkManager<T>
where
    T: Proxy + Sync,
{
    /// Runs the network manager, processing incoming events.
    pub async fn run(mut self) {
        while let Some(event) = self.events_in.recv().await {
            // TODO: forward unhandled events.
            match event {
                Event::DeviceJoined {
                    ieee_address,
                    pan_id,
                } => {
                    info!("Device joined: IEEE Address: {ieee_address}, PAN ID: {pan_id:#06X}");
                    self.state
                        .add_node(Node::new(ieee_address, pan_id, Descriptor::default()));

                    if let Err(error) = self.send_bind_req(ieee_address, pan_id, 1, 0x0006).await {
                        error!("Failed to send bind request for endpoint 1: {error}");
                    }

                    if let Err(error) = self.send_bind_req(ieee_address, pan_id, 2, 0x0006).await {
                        error!("Failed to send bind request for endpoint 2: {error}");
                    }

                    if let Err(error) = self.send_bind_req(ieee_address, pan_id, 1, 0x0008).await {
                        error!("Failed to send bind request for endpoint 1: {error}");
                    }

                    if let Err(error) = self.send_bind_req(ieee_address, pan_id, 2, 0x0008).await {
                        error!("Failed to send bind request for endpoint 2: {error}");
                    }
                }
                Event::DeviceRejoined {
                    ieee_address,
                    pan_id,
                    secured,
                } => {
                    info!(
                        "Device rejoined: IEEE Address: {ieee_address}, PAN ID: {pan_id:#06X}, Secured: {secured}"
                    );
                    self.state
                        .add_node(Node::new(ieee_address, pan_id, Descriptor::default()));
                }
                Event::DeviceLeft {
                    ieee_address,
                    pan_id,
                } => {
                    info!("Device left: IEEE Address: {ieee_address}, PAN ID: {pan_id:#06X}");
                    self.state.remove_node(pan_id);
                }
                Event::MessageReceived {
                    src_address,
                    src_endpoint,
                    command,
                    ..
                } => match *command {
                    Command::Zdp(command) => {
                        if let Err(error) = self.handle_zdp_command(src_address, command).await {
                            error!("Failed to handle ZDP command: {error}");
                        }
                    }
                    Command::Zcl(command) => {
                        self.handle_zcl_command(src_address, src_endpoint, command)
                            .await;
                    }
                },
                other => {
                    warn!("Unhandled event: {other:?}");
                }
            }
        }
    }

    async fn send_bind_req(
        &self,
        ieee_address: MacAddr8,
        pan_id: u16,
        src_endpoint: u8,
        cluster_id: u16,
    ) -> Result<(), Error> {
        let dst_address = self.proxy.get_ieee_address(0x0000).await?;
        info!(
            "Requesting bind to {pan_id} of {ieee_address}/{src_endpoint} to {dst_address}/1 for cluster {cluster_id:#06X}"
        );
        self.proxy
            .zdp()
            .unicast(
                pan_id,
                Endpoint::Data,
                BindReq::new(
                    ieee_address,
                    src_endpoint,
                    cluster_id,
                    Destination::Extended {
                        address: dst_address,
                        endpoint: 1,
                    },
                ),
            )
            .await?;

        Ok(())
    }

    async fn handle_zdp_command(
        &self,
        src_address: u16,
        command: zdp::Frame<zdp::Command>,
    ) -> Result<(), Error> {
        info!("Received ZDP command: {command:?}");
        let (seq, command) = command.into_parts();

        match command {
            zdp::Command::NetworkManagement(network_management) => match network_management {
                zdp::NetworkManagement::MgmtPermitJoiningReq(mgmt_permit_joining_req) => {
                    info!("Received Mgmt Permit Joining Request: {mgmt_permit_joining_req:?}");
                    self.proxy
                        .zdp()
                        .unicast(
                            src_address,
                            Endpoint::Data,
                            zdp::MgmtPermitJoiningRsp::new(zdp::Status::Success),
                        )
                        .await?;
                }
                command => {
                    warn!(
                        "Received unhandled Network Management command (seq: {seq}): {command:?}"
                    );
                }
            },
            zdp::Command::DeviceAndServiceDiscovery(device_and_service) => match device_and_service
            {
                zdp::DeviceAndServiceDiscovery::MatchDescReq(match_desc_req) => {
                    info!("Received Match Descriptor Request: {match_desc_req:?}");
                    self.proxy
                        .zdp()
                        .unicast(
                            src_address,
                            Endpoint::Data,
                            zdp::MatchDescRsp::new(zdp::Status::Success, 0x0000, vec![0x01].into())
                                .expect("Endpoint list fits."),
                        )
                        .await?;
                }
                zdp::DeviceAndServiceDiscovery::NodeDescReq(node_desc_req) => {
                    info!("Received Node Descriptor Request: {node_desc_req:?}");
                    self.proxy
                        .zdp()
                        .unicast(
                            src_address,
                            Endpoint::Data,
                            zdp::NodeDescRsp::new(
                                node_desc_req.nwk_addr(),
                                zdp::Status::Success,
                                create_node_descriptor(),
                                vec![],
                            ),
                        )
                        .await?;
                }
                command => {
                    warn!(
                        "Received unhandled Device and Service Discovery command (seq: {seq}): {command:?}"
                    );
                }
            },
            zdp::Command::BindManagement(_) => {
                warn!("Received unhandled ZDP command (seq: {seq}): {command:?}");
            }
        }

        Ok(())
    }

    async fn handle_zcl_command(
        &self,
        src_address: u16,
        src_endpoint: Endpoint,
        command: zcl::Frame<Cluster>,
    ) {
        info!("Received ZCL command from {src_address:#06X}/{src_endpoint}: {command:?}");
        let (_header, payload) = command.into_parts();

        if let Cluster::OnOff(on_off) = payload {
            match on_off {
                on_off::Command::On(_) => {
                    for node in self.state.iter_nodes() {
                        for endpoint in 1..4 {
                            debug!("Sending On command to {:#06X}/{}", node.pan_id(), endpoint);
                            self.proxy
                                .zcl()
                                .unicast(node.pan_id(), endpoint.into(), on_off::On)
                                .await
                                .expect("Failed to send On command.");
                        }
                    }
                }
                on_off::Command::Off(_) => {
                    for node in self.state.iter_nodes() {
                        for endpoint in 1..4 {
                            debug!("Sending Off command to {:#06X}/{}", node.pan_id(), endpoint);
                            self.proxy
                                .zcl()
                                .unicast(node.pan_id(), endpoint.into(), on_off::Off)
                                .await
                                .expect("Failed to send Off command.");
                        }
                    }
                }
                other => {
                    warn!("Received unhandled On/Off command: {other:?}");
                }
            }
        }
    }
}

/// Creates a node descriptor for the coordinator.
fn create_node_descriptor() -> Descriptor {
    let mut flags = Flags::empty();
    flags.set_frequency_band(FrequencyBand::FROM_2400_TO_2483_5_MHZ);
    flags.set_logical_type(LogicalType::Coordinator);

    let mac_capability_flags = MacCapabilityFlags::ALTERNATE_PAN_COORDINATOR
        | MacCapabilityFlags::DEVICE_TYPE
        | MacCapabilityFlags::POWER_SOURCE
        | MacCapabilityFlags::RECEIVER_ON_WHEN_IDLE
        | MacCapabilityFlags::SECURITY_CAPABLE
        | MacCapabilityFlags::ALLOCATE_ADDRESS;

    let mut server_mask = ServerMask::NETWORK_MANAGER | ServerMask::PRIMARY_TRUST_CENTER;
    server_mask.set_stack_compliance_revision(23);

    Descriptor::new(
        flags,
        mac_capability_flags,
        0x1218,
        0x7f,
        0x7fff,
        server_mask,
        0x7fff,
    )
}
