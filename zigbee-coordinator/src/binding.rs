use std::collections::BTreeMap;

use log::{error, warn};
use tokio::spawn;
use tokio::sync::mpsc::{Receiver, Sender, channel};
use tokio::time::sleep;
use zdp::{BindReq, Destination, Status};
use zigbee::{Address, ClusterId, Endpoint};

use self::devices_ext::{Devices, DevicesExt};
pub use self::message::Message;
use crate::discovery::EndpointInfo;
use crate::transceiver::zdp::Handle;
use crate::{MAX_RETRIES, RETRY_DELAY, network_manager, transceiver};

mod devices_ext;
mod message;

/// The output clusters that the coordinator binds to.
const BIND_OUTPUT_CLUSTERS: [ClusterId; 2] = [ClusterId::OnOff, ClusterId::Level];

/// The binding management actor.
#[derive(Debug)]
pub struct Actor {
    inbox: Receiver<Message>,
    loopback: Sender<Message>,
    zdp: Sender<transceiver::zdp::Message>,
    network_manager: Sender<network_manager::Message>,
    coordinator_address: Address,
    devices: Devices,
}

impl Actor {
    /// Create a new binding management actor.
    #[must_use]
    pub fn new(
        buffer: usize,
        zdp: Sender<transceiver::zdp::Message>,
        network_manager: Sender<network_manager::Message>,
        coordinator_address: Address,
    ) -> (Self, Sender<Message>) {
        let (tx, rx) = channel(buffer);

        let instance = Self {
            inbox: rx,
            loopback: tx.clone(),
            zdp,
            network_manager,
            coordinator_address,
            devices: BTreeMap::new(),
        };

        (instance, tx)
    }

    /// Run the binding management actor.
    pub async fn run(mut self) {
        while let Some(message) = self.inbox.recv().await {
            match message {
                Message::DeviceDiscovered { address, endpoints } => {
                    self.bind_device_endpoints(&address, endpoints);
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

    fn bind_device_endpoints(
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
                    spawn(bind_endpoint(
                        address.clone(),
                        *endpoint,
                        cluster,
                        self.loopback.clone(),
                        self.zdp.clone(),
                        self.coordinator_address.clone(),
                    ));
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
            self.network_manager
                .send(network_manager::Message::NewDevice { address, endpoints })
                .await
                .unwrap_or_else(|error| {
                    error!("Failed to send new device message: {error:?}");
                });
        }
    }
}

/// Run the binding loop.
#[expect(clippy::too_many_arguments)]
async fn bind_endpoint(
    address: Address,
    endpoint: Endpoint,
    cluster: ClusterId,
    loopback: Sender<Message>,
    zdp: Sender<transceiver::zdp::Message>,
    coordinator_address: Address,
) {
    let short_id = address.short_id();
    let mut retries = 0;

    loop {
        if retries > MAX_RETRIES {
            error!(
                "Failed to bind {address}:{endpoint} to cluster {cluster:?} after {MAX_RETRIES} retries. Giving up."
            );
            return;
        }

        if retries > 0 {
            sleep(RETRY_DELAY).await;
        }

        retries += 1;

        match zdp
            .communicate(
                short_id,
                BindReq::new(
                    address.ieee_address(),
                    endpoint.into(),
                    cluster.into(),
                    Destination::Extended {
                        address: coordinator_address.ieee_address(),
                        endpoint: Endpoint::default().into(),
                    },
                ),
            )
            .await
        {
            Ok(response) => {
                if response.status() == Ok(Status::Success) {
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
}
