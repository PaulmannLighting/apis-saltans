use ::zcl::Cluster;
use ::zdp::SimpleDescriptor;
use tokio::sync::mpsc::{Sender, channel};
use zigbee::{Application, ExpectResponse};
use zigbee_hw::{Error, NcpHandle, Start};

use crate::mux::Mux;
use crate::transceiver::zcl::Payload;
use crate::transceiver::{zcl, zdp};
use crate::{MPSC_CHANNEL_SIZE, State, binding, discovery, network_manager};

/// External Zigbee API struct.
#[derive(Clone, Debug)]
pub struct Coordinator {
    pub(crate) ncp: NcpHandle,
    pub(crate) zcl: Sender<zcl::Message>,
    pub(crate) network_manager: Sender<network_manager::Message>,
}

impl Coordinator {
    /// Start the coordinator on the given hardware.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if setting up the actor network fails.
    pub async fn start<T>(
        hardware: T,
        endpoints: &[SimpleDescriptor],
        state: impl Into<State>,
    ) -> Result<Self, Error>
    where
        T: Start,
    {
        let (ncp, events) = hardware.start(endpoints).await?;
        let state = state.into();

        let (discovery_tx, discovery_rx) = channel(MPSC_CHANNEL_SIZE);
        let network_manager =
            network_manager::Actor::spawn(ncp.clone(), discovery_tx.downgrade(), state);

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
        );

        Mux::spawn(
            events,
            zcl_tx.clone(),
            zdp_tx,
            discovery_tx,
            network_manager.clone(),
        );

        Ok(Self {
            ncp,
            zcl: zcl_tx,
            network_manager,
        })
    }
}

impl zcl::Handle for Coordinator {
    async fn unicast(
        &self,
        short_id: u16,
        endpoint: Application,
        payload: Payload<Cluster>,
    ) -> Result<(), crate::Error> {
        self.zcl.unicast(short_id, endpoint, payload).await
    }

    async fn multicast(
        &self,
        group_id: u16,
        hops: u8,
        radius: u8,
        payload: Payload<Cluster>,
    ) -> Result<(), crate::Error> {
        self.zcl.multicast(group_id, hops, radius, payload).await
    }

    fn communicate<T>(
        &self,
        short_id: u16,
        endpoint: Application,
        payload: Payload<T>,
    ) -> impl Future<Output = Result<T::Response, crate::Error>> + Send
    where
        T: ExpectResponse<Cluster>,
    {
        self.zcl.communicate(short_id, endpoint, payload)
    }
}
