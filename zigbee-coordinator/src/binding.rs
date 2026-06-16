use std::collections::BTreeMap;

use log::{error, warn};
use tokio::sync::mpsc::{Receiver, Sender, WeakSender, channel};
use tokio_task_pool::Pool;
use zdp::{BindReq, Destination, Status};
use zigbee::{Address, ClusterId, Endpoint};
use zigbee_hw::{Ncp, NcpHandle, WeakNcpHandle};

pub use self::devices_ext::Devices;
use self::devices_ext::DevicesExt;
pub use self::message::Message;
use crate::discovery::EndpointInfo;
use crate::transceiver::zdp::Handle;
use crate::{MPSC_CHANNEL_SIZE, RETRY, TASK_POOL_SIZE, network_manager, transceiver};

mod devices_ext;
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
                    self.bind_device_endpoints(&address, endpoints).await;
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
        address: &Address,
        endpoints: BTreeMap<Endpoint, EndpointInfo>,
    ) {
        let device = self.devices.update(address.clone(), endpoints);

        for (endpoint, (endpoint_info, _)) in device.get() {
            for cluster in BIND_OUTPUT_CLUSTERS {
                if endpoint_info
                    .descriptor()
                    .output_clusters()
                    .contains(&cluster.into())
                {
                    self.tasks
                        .spawn(bind_endpoint(
                            address.clone(),
                            *endpoint,
                            cluster,
                            self.loopback.clone(),
                            self.zdp.clone(),
                            self.ncp.clone(),
                        ))
                        .await
                        .map_or_else(drop, |error| {
                            error!("Failed to spawn task: {error:?}");
                        });
                }
            }
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
            let Some(network_manager) = self.network_manager.upgrade() else {
                return;
            };

            network_manager
                .send(network_manager::Message::NewDevice(
                    (address, endpoints).into(),
                ))
                .await
                .unwrap_or_else(|error| {
                    error!("Failed to send new device message: {error:?}");
                });
        }
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
    let short_id = address.short_id();
    let mut retries = 0;

    while RETRY.retry(&mut retries).await {
        let Some(zdp) = zdp.upgrade() else {
            return;
        };

        let Some(ncp_handle) = ncp.upgrade() else {
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
                        return;
                    };

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
