use std::collections::BTreeMap;

use log::{error, trace, warn};
use tokio::sync::mpsc::{Receiver, Sender, WeakSender, channel};
use tokio_task_pool::Pool;
use zdp::{BindReq, Destination, Status};
use zigbee::{Address, ClusterId, Endpoint};
use zigbee_hw::{Ncp, WeakNcpHandle};

use self::clusters_to_bind::ClustersToBind;
pub use self::devices_ext::Devices;
use self::devices_ext::DevicesExt;
pub use self::message::Message;
use crate::binding::needs_binding::NeedsBinding;
use crate::transceiver::zdp::Handle;
use crate::{
    Device, Endpoint as EndpointInfo, MPSC_CHANNEL_SIZE, RETRY, TASK_POOL_SIZE, network_manager,
    transceiver,
};

mod clusters_to_bind;
mod devices_ext;
mod message;
mod needs_binding;

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
    devices: Devices,
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

    /// Run the binding management actor.
    pub async fn run(mut self) {
        while let Some(message) = self.inbox.recv().await {
            match message {
                Message::DeviceDiscovered { address, endpoints } => {
                    self.bind_device_endpoints(address, endpoints).await;
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

    async fn bind_device_endpoints(
        &mut self,
        address: Address,
        endpoints: BTreeMap<Endpoint, EndpointInfo>,
    ) {
        trace!("Binding device {address} with endpoints: {endpoints:?}");

        if !endpoints.needs_binding(&BIND_OUTPUT_CLUSTERS) {
            trace!("No binding necessary for {address}.");
            self.forward_device(address, endpoints).await;
            return;
        }

        let device = self.devices.update(address.clone(), endpoints);

        for (endpoint, cluster) in device.get().clusters_to_bind(&BIND_OUTPUT_CLUSTERS) {
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
        self.devices.endpoint_bound(&address, endpoint, cluster);
        self.forward_device_if_complete(address).await;
    }

    async fn forward_device_if_complete(&mut self, address: Address) {
        if let Some(endpoints) = self.devices.remove_if_binding_complete(&address) {
            trace!("Device {address} is now bound: {endpoints:?}");

            trace!("Forwarding device {address} to network manager.");
            self.forward_device(address, endpoints).await;
        } else {
            trace!("Not all endpoints bound for {address}.");
        }
    }

    async fn forward_device(&self, address: Address, endpoints: BTreeMap<Endpoint, EndpointInfo>) {
        let Some(network_manager) = self.network_manager.upgrade() else {
            trace!("Network manager channel closed. Aborting forwarding of device: {address}.");
            return;
        };
        network_manager
            .send(network_manager::Message::NewDevice(
                Device::from((address, endpoints)).into(),
            ))
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
                    endpoint.into(),
                    cluster.into(),
                    Destination::Extended {
                        address: coordinator_address,
                        endpoint: Endpoint::default().into(),
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
