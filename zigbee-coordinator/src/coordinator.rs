use tokio::spawn;
use tokio::sync::mpsc::{Receiver, Sender, WeakSender, channel};
use zigbee::Address;
use zigbee_hw::{Error, Event, Ncp, NcpHandle, Start, bridge};

use crate::mux::{Handle as MuxHandle, Mux};
use crate::transceiver::{zcl, zdp};
use crate::{MPSC_CHANNEL_SIZE, binding, discovery, mux, network_manager};

/// External Zigbee API struct.
#[derive(Clone, Debug)]
pub struct Coordinator {
    pub(crate) zcl_transceiver: Sender<zcl::Message>,
    pub(crate) zdp_transceiver: Sender<zdp::Message>,
    pub(crate) network_manager: Sender<network_manager::Message>,
    pub(crate) binding_manager: Sender<binding::Message>,
    pub(crate) mux: Sender<mux::Message>,
}

impl Coordinator {
    /// Start the coordinator on the given hardware.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if setting up the actor network fails.
    pub async fn start<T>(hardware: T) -> Result<Self, Error>
    where
        T: Start,
    {
        let (ncp, events) = hardware.start().await?;
        let coordinator_address = ncp.get_address().await?;
        let zcl_tx = Self::start_zcl_transceiver(ncp.clone());
        let zdp_tx = Self::start_zdp_transceiver(ncp);
        let (mux, discovery_rx) = Self::start_mux(events, zcl_tx.clone(), zdp_tx.clone());
        let network_manager = Self::start_network_manager(&mux).await?;
        let binding_manager = Self::start_binding_manager(
            zdp_tx.downgrade(),
            network_manager.downgrade(),
            coordinator_address,
        );
        Self::start_discovery_manager(
            discovery_rx,
            zcl_tx.downgrade(),
            zdp_tx.downgrade(),
            binding_manager.downgrade(),
        );

        Ok(Self {
            zcl_transceiver: zcl_tx,
            zdp_transceiver: zdp_tx,
            network_manager,
            binding_manager,
            mux,
        })
    }

    /// Start the multiplexer.
    fn start_mux(
        events: Receiver<Event>,
        zcl_tx: Sender<zcl::Message>,
        zdp_tx: Sender<zdp::Message>,
    ) -> (Sender<mux::Message>, Receiver<discovery::Message>) {
        let (mux_tx, mux_rx) = channel(MPSC_CHANNEL_SIZE);
        let (discovery_tx, discovery_rx) = channel(MPSC_CHANNEL_SIZE);
        spawn(bridge(events, mux_tx.clone()));
        spawn(Mux::new(discovery_tx, zcl_tx, zdp_tx).run(mux_rx));
        (mux_tx, discovery_rx)
    }

    /// Start the ZCL transceiver.
    fn start_zcl_transceiver(ncp: NcpHandle) -> Sender<zcl::Message> {
        let (zcl_tx, zcl_rx) = channel(MPSC_CHANNEL_SIZE);
        spawn(zcl::Transceiver::new(ncp).run(zcl_rx));
        zcl_tx
    }

    /// Start the ZDP transceiver.
    fn start_zdp_transceiver(ncp: NcpHandle) -> Sender<zdp::Message> {
        let (zdp_tx, zdp_rx) = channel(MPSC_CHANNEL_SIZE);
        spawn(zdp::Transceiver::new(ncp).run(zdp_rx));
        zdp_tx
    }

    /// Start the network manager.
    async fn start_network_manager(
        mux: &Sender<mux::Message>,
    ) -> Result<Sender<network_manager::Message>, Error> {
        let network_manager = network_manager::Actor::new();
        let (network_manager_tx, network_manager_rx) = channel(MPSC_CHANNEL_SIZE);
        let (events_tx, events_rx) = channel(MPSC_CHANNEL_SIZE);
        mux.subscribe(events_tx).await?;
        spawn(bridge(events_rx, network_manager_tx.clone()));
        spawn(network_manager.run(network_manager_rx));
        Ok(network_manager_tx)
    }

    /// Start the binding manager.
    fn start_binding_manager(
        zdp_transceiver: WeakSender<zdp::Message>,
        network_manager: WeakSender<network_manager::Message>,
        coordinator_address: Address,
    ) -> Sender<binding::Message> {
        let (binding_manager, binding_manager_tx) =
            binding::Actor::new(zdp_transceiver, network_manager, coordinator_address);
        spawn(binding_manager.run());
        binding_manager_tx
    }

    /// Start the discovery manager.
    fn start_discovery_manager(
        discovery_rx: Receiver<discovery::Message>,
        zcl_transmitter: WeakSender<zcl::Message>,
        zdp_transmitter: WeakSender<zdp::Message>,
        binding_manager: WeakSender<binding::Message>,
    ) {
        let discovery_manager =
            discovery::Actor::new(zcl_transmitter, zdp_transmitter, binding_manager);
        spawn(discovery_manager.run(discovery_rx));
    }
}
