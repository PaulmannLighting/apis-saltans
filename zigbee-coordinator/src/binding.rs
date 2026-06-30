use std::collections::BTreeMap;

use log::{error, trace, warn};
use tokio::spawn;
use tokio::sync::mpsc::{Receiver, Sender, WeakSender, channel};
use tokio_task_pool::Pool;
use zdp::{BindReq, Destination, Status};
use zigbee::{Address, ClusterId, Endpoint};
use zigbee_hw::{Ncp, WeakNcpHandle};

pub use self::device::Device;
pub use self::message::Message;
use crate::transceiver::zdp::Handle;
use crate::{MPSC_CHANNEL_SIZE, RETRY, TASK_POOL_SIZE, network_manager, transceiver};

mod device;
mod message;

/// The output clusters that the coordinator binds to.
const BIND_OUTPUT_CLUSTERS: [ClusterId; 2] = [ClusterId::OnOff, ClusterId::Level];

/// The binding management actor.
#[derive(Debug)]
pub struct Actor {
    inbox: Receiver<Message>,
    loopback: WeakSender<Message>,
    zdp: WeakSender<transceiver::zdp::Message>,
    network_manager: WeakSender<network_manager::Message>,
    ncp: WeakNcpHandle,
    devices: BTreeMap<Address, Device>,
    tasks: Pool,
}

impl Actor {
    /// Create a new binding management actor.
    #[must_use]
    pub fn new(
        zdp: WeakSender<transceiver::zdp::Message>,
        network_manager: WeakSender<network_manager::Message>,
        ncp: WeakNcpHandle,
    ) -> (Self, Sender<Message>) {
        let (tx, rx) = channel(MPSC_CHANNEL_SIZE);

        let instance = Self {
            inbox: rx,
            loopback: tx.downgrade(),
            zdp,
            network_manager,
            ncp,
            devices: BTreeMap::new(),
            tasks: Pool::bounded(TASK_POOL_SIZE),
        };

        (instance, tx)
    }

    /// Start the binding manager.
    pub fn spawn(
        zdp_tx: WeakSender<transceiver::zdp::Message>,
        network_manager: WeakSender<network_manager::Message>,
        ncp: WeakNcpHandle,
    ) -> Sender<Message> {
        let (binding_manager, binding_manager_tx) = Self::new(zdp_tx, network_manager, ncp);
        spawn(binding_manager.run());
        binding_manager_tx
    }

    /// Run the binding management actor.
    pub async fn run(mut self) {
        while let Some(message) = self.inbox.recv().await {
            match message {
                Message::DeviceDiscovered(device) => {
                    self.bind_device_endpoints((*device).into()).await;
                }
                Message::EndpointBound {
                    address,
                    endpoint,
                    cluster,
                } => {
                    self.update_bound_endpoints(address, endpoint, cluster)
                        .await;
                }
            }
        }
    }

    async fn bind_device_endpoints(&mut self, device: Device) {
        trace!("Binding device: {device}");

        if !device.needs_binding(&BIND_OUTPUT_CLUSTERS) {
            trace!("No binding necessary for {device}.");
            self.forward_device(device).await;
            return;
        }

        let address = device.address.clone();
        let device = self.devices.entry(address.clone()).or_insert(device);

        for (endpoint, cluster) in device.clusters_to_bind(&BIND_OUTPUT_CLUSTERS) {
            self.tasks
                .spawn(bind_endpoint(
                    address.clone(),
                    endpoint,
                    cluster,
                    self.loopback.clone(),
                    self.zdp.clone(),
                    self.ncp.clone(),
                ))
                .await
                .map_or_else(
                    |error| {
                        error!("Failed to spawn task: {error:?}");
                    },
                    drop,
                );
        }
    }

    async fn update_bound_endpoints(
        &mut self,
        address: Address,
        endpoint: Endpoint,
        cluster: ClusterId,
    ) {
        let Some(mut device) = self.devices.remove(&address) else {
            trace!("No device found for {address}.");
            return;
        };

        let Some(endpoint) = device.endpoints.get_mut(&endpoint) else {
            trace!("No endpoint found for {address}:{endpoint}.");
            return;
        };

        endpoint.bound_clusters.insert(cluster);

        if device.needs_binding(&BIND_OUTPUT_CLUSTERS) {
            trace!("Not all endpoints bound for {address}.");
            self.devices.insert(device.address.clone(), device);
            return;
        }

        trace!("Device {address} is now bound.");
        self.forward_device(device).await;
    }

    async fn forward_device(&self, device: Device) {
        let Some(network_manager) = self.network_manager.upgrade() else {
            trace!("Network manager channel closed.");
            return;
        };

        trace!("Forwarding device {device} to network manager.");
        network_manager
            .send(network_manager::Message::NewDevice(device.into()))
            .await
            .unwrap_or_else(|error| {
                error!("Failed to send new device message: {error:?}");
            });
    }
}

/// Run the binding loop.
async fn bind_endpoint(
    address: Address,
    endpoint: Endpoint,
    cluster: ClusterId,
    loopback: WeakSender<Message>,
    zdp: WeakSender<transceiver::zdp::Message>,
    ncp: WeakNcpHandle,
) {
    trace!("Binding {address}:{endpoint} to cluster {cluster}.");
    let short_id = address.short_id();
    let mut retries = 0;

    while RETRY.retry(&mut retries).await {
        let Some(zdp) = zdp.upgrade() else {
            trace!("Failed to upgrade ZDP sender.");
            return;
        };

        let Some(ncp_handle) = ncp.upgrade() else {
            trace!("Failed to upgrade NCP handle.");
            return;
        };

        let Ok(coordinator_address) = ncp_handle.get_ieee_address().await.map_err(|error| {
            error!("Failed to get coordinator address: {error:?}");
        }) else {
            return;
        };

        match zdp
            .communicate(
                short_id,
                BindReq::new(
                    address.ieee_address(),
                    endpoint,
                    cluster.into(),
                    Destination::Extended {
                        address: coordinator_address,
                        endpoint: Endpoint::default(),
                    },
                ),
            )
            .await
        {
            Ok(response) => {
                if response.status() == Ok(Status::Success) {
                    let Some(loopback) = loopback.upgrade() else {
                        trace!("Failed to upgrade loopback sender.");
                        return;
                    };

                    trace!(
                        "Bound {address}:{endpoint} to cluster {cluster}. Forwarding to loopback."
                    );
                    loopback
                        .send(Message::EndpointBound {
                            address,
                            endpoint,
                            cluster,
                        })
                        .await
                        .unwrap_or_else(|error| {
                            error!("Failed to send endpoint bound message: {error:?}");
                        });
                    return;
                }

                warn!("Failed to bind {address}:{endpoint} to cluster {cluster:?}: {response:?}");
            }
            Err(error) => {
                warn!("Failed to bind {address}:{endpoint} to cluster {cluster:?}: {error:?}");
            }
        }
    }

    error!("Failed to bind {address}:{endpoint} to cluster {cluster:?}.");
}
