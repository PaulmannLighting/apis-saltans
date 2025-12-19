use log::{error, info, warn};
use macaddr::MacAddr8;
use tokio::sync::mpsc::{Receiver, Sender, channel};
use zcl::Cluster;
use zcl::general::on_off;
use zdp::{BindReq, Destination};
use zigbee::Endpoint;
use zigbee::node::{Descriptor, Node};

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
                    Command::Zdp(zdp_command) => {
                        info!("Received ZDP command: {zdp_command:?}");
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

    async fn handle_zcl_command(
        &self,
        src_address: u16,
        src_endpoint: Endpoint,
        command: zcl::Frame<Cluster>,
    ) {
        info!("Received ZCL command from {src_address:#06X}/{src_endpoint}: {command:?}");
        let (_header, payload) = command.into_parts();

        if let Cluster::OnOff(on_off) = payload {
            let neighbors = self.proxy.get_neighbors().await.unwrap_or_default();

            match on_off {
                on_off::Command::On(_) => {
                    for node in self.state.iter_nodes() {
                        self.proxy
                            .zcl()
                            .unicast(node.pan_id(), 0x01.into(), on_off::On)
                            .await
                            .expect("Failed to send On command.");
                    }
                }
                on_off::Command::Off(_) => {
                    for node in self.state.iter_nodes() {
                        self.proxy
                            .zcl()
                            .unicast(node.pan_id(), 0x01.into(), on_off::Off)
                            .await
                            .expect("Failed to send Off command.");
                    }
                }
                other => {
                    warn!("Received unhandled On/Off command: {other:?}");
                }
            }
        }
    }
}
