use std::fmt::Debug;

use le_stream::ToLeStream;
use tokio::sync::mpsc::{Receiver, Sender, channel};
use zb_core::destination::Device;
use zb_core::{ClusterSpecific, Destination, ExpectResponse, Profiled};
use zb_hw::{Error, Event, NcpHandle};
use zb_zcl::{Cluster, Command};
use zb_zdp::SimpleDescriptor;

use crate::mux::Mux;
use crate::transceiver::zcl::Payload;
use crate::transceiver::{zcl, zdp};
use crate::{MPSC_CHANNEL_SIZE, binding, discovery, network_manager, storage};

/// External Zigbee API struct.
#[derive(Clone, Debug)]
pub struct Coordinator {
    pub(crate) ncp: NcpHandle,
    pub(crate) zcl: Sender<zcl::Message>,
    pub(crate) network_manager: Sender<network_manager::Message>,
    pub(crate) discovery_manager: Sender<discovery::Message>,
}

impl Coordinator {
    /// Start the coordinator on the given hardware.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if setting up the actor network fails.
    pub async fn start(
        ncp: NcpHandle,
        events: Receiver<Event>,
        storage: Sender<storage::Message>,
        endpoints: &[SimpleDescriptor],
    ) -> Result<Self, Error> {
        let (discovery_tx, discovery_rx) = channel(MPSC_CHANNEL_SIZE);
        let network_manager = network_manager::Actor::spawn(ncp.clone(), storage);

        let zcl_tx = zcl::Transceiver::spawn(ncp.clone(), network_manager.downgrade());
        let zdp_tx = zdp::Transceiver::spawn(ncp.clone(), discovery_tx.downgrade(), endpoints);

        let binding_manager = binding::Actor::spawn(
            zdp_tx.downgrade(),
            network_manager.downgrade(),
            ncp.downgrade(),
        );

        discovery::Actor::spawn(
            discovery_rx,
            zcl_tx.downgrade(),
            zdp_tx.downgrade(),
            binding_manager,
            ncp.downgrade(),
        );

        Mux::spawn(events, zcl_tx.clone(), zdp_tx, network_manager.clone());

        Ok(Self {
            ncp,
            zcl: zcl_tx,
            network_manager,
            discovery_manager: discovery_tx,
        })
    }
}

impl zcl::Handle for Coordinator {
    async fn transmit<T>(&self, destination: Destination, payload: T) -> Result<(), crate::Error>
    where
        T: ClusterSpecific + Command + Debug + Profiled + ToLeStream,
    {
        self.zcl.transmit(destination, payload).await
    }

    async fn communicate<T>(
        &self,
        destination: Device,
        payload: T,
    ) -> Result<T::Response, crate::Error>
    where
        T: ExpectResponse<Cluster> + Into<Payload> + Send,
    {
        self.zcl.communicate(destination, payload).await
    }
}
