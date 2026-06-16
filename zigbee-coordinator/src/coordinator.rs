use tokio::spawn;
use tokio::sync::mpsc::{Receiver, Sender, WeakSender, channel};
use zigbee::Address;
use zigbee_hw::{Error, Event, Ncp, NcpHandle, Start, bridge};

use crate::mux::{Handle as MuxHandle, Mux};
use crate::{MPSC_CHANNEL_SIZE, binding, discovery, mux, network_manager, transceiver};

/// External Zigbee API struct.
#[derive(Clone, Debug)]
pub struct Coordinator {
    pub(crate) zcl_transceiver: Sender<transceiver::zcl::Message>,
    pub(crate) zdp_transceiver: Sender<transceiver::zdp::Message>,
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
        let mux = Self::start_mux(events);
        let zcl_transceiver = Self::start_zcl_transceiver(ncp.clone(), &mux).await?;
        let zdp_transceiver = Self::start_zdp_transceiver(ncp, &mux).await?;
        let network_manager = Self::start_network_manager(&mux).await?;
        let binding_manager = Self::start_binding_manager(
            zdp_transceiver.downgrade(),
            network_manager.downgrade(),
            coordinator_address,
        );
        Self::start_discovery_manager(
            &mux,
            zcl_transceiver.downgrade(),
            zdp_transceiver.downgrade(),
            binding_manager.downgrade(),
        )
        .await?;

        Ok(Self {
            zcl_transceiver,
            zdp_transceiver,
            network_manager,
            binding_manager,
            mux,
        })
    }

    /// Start the multiplexer.
    fn start_mux(events: Receiver<Event>) -> Sender<mux::Message> {
        let (mux_tx, mux_rx) = channel(MPSC_CHANNEL_SIZE);
        spawn(bridge(events, mux_tx.clone()));
        spawn(Mux::default().run(mux_rx));
        mux_tx
    }

    /// Start the ZCL transceiver.
    async fn start_zcl_transceiver(
        ncp: NcpHandle,
        mux: &Sender<mux::Message>,
    ) -> Result<Sender<transceiver::zcl::Message>, Error> {
        let transceiver = transceiver::zcl::Transceiver::new(ncp);
        let (transceiver_tx, transceiver_rx) = channel(MPSC_CHANNEL_SIZE);
        let (events_tx, events_rx) = channel(MPSC_CHANNEL_SIZE);
        mux.subscribe(events_tx).await?;
        spawn(bridge(events_rx, transceiver_tx.clone()));
        spawn(transceiver.run(transceiver_rx));
        Ok(transceiver_tx)
    }

    /// Start the ZDP transceiver.
    async fn start_zdp_transceiver(
        ncp: NcpHandle,
        mux: &Sender<mux::Message>,
    ) -> Result<Sender<transceiver::zdp::Message>, Error> {
        let transceiver = transceiver::zdp::Transceiver::new(ncp);
        let (transceiver_tx, transceiver_rx) = channel(MPSC_CHANNEL_SIZE);
        let (events_tx, events_rx) = channel(MPSC_CHANNEL_SIZE);
        mux.subscribe(events_tx).await?;
        spawn(bridge(events_rx, transceiver_tx.clone()));
        spawn(transceiver.run(transceiver_rx));
        Ok(transceiver_tx)
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
        zdp_transceiver: WeakSender<transceiver::zdp::Message>,
        network_manager: WeakSender<network_manager::Message>,
        coordinator_address: Address,
    ) -> Sender<binding::Message> {
        let (binding_manager, binding_manager_tx) =
            binding::Actor::new(zdp_transceiver, network_manager, coordinator_address);
        spawn(binding_manager.run());
        binding_manager_tx
    }

    /// Start the discovery manager.
    async fn start_discovery_manager(
        mux: &Sender<mux::Message>,
        zcl_transmitter: WeakSender<transceiver::zcl::Message>,
        zdp_transmitter: WeakSender<transceiver::zdp::Message>,
        binding_manager: WeakSender<binding::Message>,
    ) -> Result<(), Error> {
        let discovery_manager =
            discovery::Actor::new(zcl_transmitter, zdp_transmitter, binding_manager);
        let (events_tx, events_rx) = channel(MPSC_CHANNEL_SIZE);
        mux.subscribe(events_tx).await?;
        spawn(discovery_manager.run(events_rx));
        Ok(())
    }
}
