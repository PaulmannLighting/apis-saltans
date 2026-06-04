use tokio::spawn;
use tokio::sync::mpsc::{Receiver, Sender, channel};
use zigbee_hw::{Error, Event, NcpHandle, Start, bridge};

use crate::mux::{Handle as MuxHandle, Mux};
use crate::transmitter::Transmitter;
use crate::{binding, discovery, mux, network_manager, transmitter};

/// External Zigbee API struct.
#[derive(Clone, Debug)]
pub struct Api {
    transmitter: Sender<transmitter::Message>,
    network_manager: Sender<network_manager::Message>,
    binding_manager: Sender<binding::Message>,
    mux: Sender<mux::Message>,
}

impl Api {
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
        let mux_tx = Self::start_mux(events);
        let transmitter_tx = Self::start_transmitter(ncp, &mux_tx).await?;
        let network_manager_tx = Self::start_network_manager(&mux_tx).await?;
        let binding_manager_tx = Self::start_binding_manager(
            &mux_tx,
            transmitter_tx.clone(),
            network_manager_tx.clone(),
        )
        .await?;
        Self::start_discovery_manager(&mux_tx, transmitter_tx.clone(), binding_manager_tx.clone())
            .await?;

        Ok(Self {
            transmitter: transmitter_tx,
            network_manager: network_manager_tx,
            binding_manager: binding_manager_tx,
            mux: mux_tx,
        })
    }

    /// Start the multiplexer.
    fn start_mux(events: Receiver<Event>) -> Sender<mux::Message> {
        let (mux_tx, mux_rx) = channel(100);
        spawn(bridge(events, mux_tx.clone()));
        spawn(Mux::default().run(mux_rx));
        mux_tx
    }

    /// Start the transmitter
    async fn start_transmitter(
        ncp: NcpHandle,
        mux: &Sender<mux::Message>,
    ) -> Result<Sender<transmitter::Message>, Error> {
        let transmitter = Transmitter::new(ncp);
        let (transmitter_tx, transmitter_rx) = channel(100);
        let (events_tx, events_rx) = channel(100);
        mux.subscribe(events_tx).await?;
        spawn(bridge(events_rx, transmitter_tx.clone()));
        spawn(transmitter.run(transmitter_rx));
        Ok(transmitter_tx)
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
        transmitter: Sender<transmitter::Message>,
        network_manager: Sender<network_manager::Message>,
    ) -> Result<Sender<binding::Message>, Error> {
        let binding_manager = binding::Actor::new(transmitter, network_manager);
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
        transmitter: Sender<transmitter::Message>,
        binding_manager: Sender<binding::Message>,
    ) -> Result<(), Error> {
        let discovery_manager = discovery::Actor::new(transmitter, binding_manager);
        let (events_tx, events_rx) = channel(100);
        mux.subscribe(events_tx).await?;
        spawn(discovery_manager.run(events_rx));
        Ok(())
    }
}
