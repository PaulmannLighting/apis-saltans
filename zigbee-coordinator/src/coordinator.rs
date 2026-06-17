use ::zdp::SimpleDescriptor;
use tokio::spawn;
use tokio::sync::mpsc::{Receiver, Sender, WeakSender, channel};
use zigbee_hw::{Error, Event, NcpHandle, Start, WeakNcpHandle};
use zigbee_persistence::State;

use crate::mux::Mux;
use crate::transceiver::{zcl, zdp};
use crate::{MPSC_CHANNEL_SIZE, binding, discovery, network_manager};

/// External Zigbee API struct.
#[derive(Clone, Debug)]
pub struct Coordinator {
    pub(crate) ncp: NcpHandle,
    pub(crate) zcl: Sender<zcl::Message>,
    pub(crate) _zdp: Sender<zdp::Message>,
    pub(crate) network_manager: Sender<network_manager::Message>,
    pub(crate) _binding_manager: Sender<binding::Message>,
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
        state: State,
    ) -> Result<Self, Error>
    where
        T: Start,
    {
        let (ncp, events) = hardware.start(endpoints).await?;

        let (discovery_tx, discovery_rx) = channel(MPSC_CHANNEL_SIZE);
        let network_manager = Self::start_network_manager(state);

        let zcl_tx = Self::start_zcl_transceiver(ncp.clone());
        let zdp_tx = Self::start_zdp_transceiver(ncp.clone(), discovery_tx.downgrade(), endpoints);

        Self::start_mux(
            events,
            zcl_tx.clone(),
            zdp_tx.clone(),
            discovery_tx,
            network_manager.clone(),
        );

        let binding_manager = Self::start_binding_manager(
            zdp_tx.downgrade(),
            network_manager.downgrade(),
            ncp.downgrade(),
        );

        Self::start_discovery_manager(
            discovery_rx,
            zcl_tx.downgrade(),
            zdp_tx.downgrade(),
            binding_manager.downgrade(),
        );

        Ok(Self {
            ncp,
            zcl: zcl_tx,
            _zdp: zdp_tx,
            network_manager,
            _binding_manager: binding_manager,
        })
    }

    /// Start the multiplexer.
    fn start_mux(
        events: Receiver<Event>,
        zcl_tx: Sender<zcl::Message>,
        zdp_tx: Sender<zdp::Message>,
        discovery_tx: Sender<discovery::Message>,
        network_manager: Sender<network_manager::Message>,
    ) {
        spawn(Mux::new(zcl_tx, zdp_tx, discovery_tx, network_manager).run(events));
    }

    /// Start the ZCL transceiver.
    fn start_zcl_transceiver(ncp: NcpHandle) -> Sender<zcl::Message> {
        let (zcl_tx, zcl_rx) = channel(MPSC_CHANNEL_SIZE);
        spawn(zcl::Transceiver::new(ncp).run(zcl_rx));
        zcl_tx
    }

    /// Start the ZDP transceiver.
    fn start_zdp_transceiver(
        ncp: NcpHandle,
        discovery: WeakSender<discovery::Message>,
        endpoints: &[SimpleDescriptor],
    ) -> Sender<zdp::Message> {
        let (zdp_tx, zdp_rx) = channel(MPSC_CHANNEL_SIZE);
        spawn(zdp::Transceiver::new(ncp, discovery, endpoints.into()).run(zdp_rx));
        zdp_tx
    }

    /// Start the network manager.
    fn start_network_manager(state: State) -> Sender<network_manager::Message> {
        let (tx, rx) = channel(MPSC_CHANNEL_SIZE);
        spawn(network_manager::Actor::new(state).run(rx));
        tx
    }

    /// Start the binding manager.
    fn start_binding_manager(
        zdp_tx: WeakSender<zdp::Message>,
        network_manager: WeakSender<network_manager::Message>,
        ncp: WeakNcpHandle,
    ) -> Sender<binding::Message> {
        let (binding_manager, binding_manager_tx) =
            binding::Actor::new(zdp_tx, network_manager, ncp);
        spawn(binding_manager.run());
        binding_manager_tx
    }

    /// Start the discovery manager.
    fn start_discovery_manager(
        discovery_rx: Receiver<discovery::Message>,
        zcl_tx: WeakSender<zcl::Message>,
        zdp_tx: WeakSender<zdp::Message>,
        binding_tx: WeakSender<binding::Message>,
    ) {
        let discovery_manager = discovery::Actor::new(zcl_tx, zdp_tx, binding_tx);
        spawn(discovery_manager.run(discovery_rx));
    }
}
