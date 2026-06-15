use tokio::spawn;
use tokio::sync::mpsc::{Receiver, Sender, channel};
use zigbee_hw::{Error, Event, NcpHandle, Start, bridge};

use crate::mux::{Handle as MuxHandle, Mux};
use crate::{binding, discovery, mux, network_manager, transceiver};

/// External Zigbee API struct.
#[derive(Clone, Debug)]
pub struct Coordinator {
    pub(crate) zcl_transceiver: Sender<transceiver::zcl::Message>,
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
        let mux = Self::start_mux(events);
        let zcl_transceiver = Self::start_zcl_transceiver(ncp.clone(), &mux).await?;
        let zdp_transceiver = Self::start_zdp_transceiver(ncp.clone(), &mux).await?;
        let network_manager = Self::start_network_manager(&mux).await?;
        let binding_manager =
            Self::start_binding_manager(&mux, zdp_transceiver.clone(), network_manager.clone())
                .await?;
        Self::start_discovery_manager(
            &mux,
            zcl_transceiver.clone(),
            zdp_transceiver,
            binding_manager.clone(),
        )
        .await?;

        Ok(Self {
            zcl_transceiver,
            network_manager,
            binding_manager,
            mux,
        })
    }

    /// Start the multiplexer.
    fn start_mux(events: Receiver<Event>) -> Sender<mux::Message> {
        let (mux_tx, mux_rx) = channel(100);
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
        let (transceiver_tx, transceiver_rx) = channel(100);
        let (events_tx, events_rx) = channel(100);
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
        let (transceiver_tx, transceiver_rx) = channel(100);
        let (events_tx, events_rx) = channel(100);
        mux.subscribe(events_tx).await?;
        spawn(bridge(events_rx, transceiver_tx.clone()));
        spawn(transceiver.run(transceiver_rx));
        Ok(transceiver_tx)
    }

    /// Start the network manager.
    async fn start_network_manager(
        mux: &Sender<mux::Message>,
    ) -> Result<Sender<network_manager::Message>, Error> {
        let network_manager = network_manager::Actor {};
        let (network_manager_tx, network_manager_rx) = channel(100);
        let (events_tx, events_rx) = channel(100);
        mux.subscribe(events_tx).await?;
        spawn(bridge(events_rx, network_manager_tx.clone()));
        spawn(network_manager.run(network_manager_rx));
        Ok(network_manager_tx)
    }

    /// Start the binding manager.
    async fn start_binding_manager(
        mux: &Sender<mux::Message>,
        zdp_transceiver: Sender<transceiver::zdp::Message>,
        network_manager: Sender<network_manager::Message>,
    ) -> Result<Sender<binding::Message>, Error> {
        let binding_manager = binding::Actor::new(zdp_transceiver, network_manager);
        let (binding_manager_tx, binding_manager_rx) = channel(100);
        let (events_tx, events_rx) = channel(100);
        mux.subscribe(events_tx).await?;
        spawn(bridge(events_rx, binding_manager_tx.clone()));
        spawn(binding_manager.run(binding_manager_rx));
        Ok(binding_manager_tx)
    }

    /// Start the discovery manager.
    async fn start_discovery_manager(
        mux: &Sender<mux::Message>,
        zcl_transmitter: Sender<transceiver::zcl::Message>,
        zdp_transmitter: Sender<transceiver::zdp::Message>,
        binding_manager: Sender<binding::Message>,
    ) -> Result<(), Error> {
        let discovery_manager =
            discovery::Actor::new(100, zcl_transmitter, zdp_transmitter, binding_manager);
        let (events_tx, events_rx) = channel(100);
        mux.subscribe(events_tx).await?;
        spawn(discovery_manager.run(events_rx));
        Ok(())
    }
}
