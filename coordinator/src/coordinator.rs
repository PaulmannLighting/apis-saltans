use std::fmt::Debug;

use tokio::sync::mpsc::{Receiver, Sender};
use zb_core::node::Descriptor;
use zb_hw::{Error, NcpHandle};

use crate::mux::Mux;
use crate::{DEFAULT_OTA_UPDATE_TASK_LIMIT, Event, ota, zcl, zdp};

/// External Zigbee API struct.
#[derive(Clone, Debug)]
pub struct Coordinator {
    pub(crate) ncp: NcpHandle,
    pub(crate) ota: Sender<ota::Message>,
    pub(crate) zcl: Sender<zcl::Message>,
    pub(crate) zdp: Sender<zdp::Message>,
}

impl Coordinator {
    /// Return a sender for the coordinator-owned OTA server inbox.
    ///
    /// Prefer the [`crate::Ota`] trait for scheduling updates. This sender is exposed for services
    /// that need to forward already parsed OTA frames into the server directly.
    #[must_use]
    pub fn ota_sender(&self) -> Sender<ota::Message> {
        self.ota.clone()
    }

    /// Start the coordinator on the given hardware.
    ///
    /// Local endpoint descriptors are obtained from the NCP through [`zb_hw::Ncp::get_endpoints`]
    /// when needed; callers do not supply them during startup.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if setting up the actor network fails.
    pub fn start(
        ncp: NcpHandle,
        descriptor: Descriptor,
        hw_events: Receiver<zb_hw::Event>,
        events_out: Sender<Event>,
    ) -> Result<Self, Error> {
        Self::start_with_ota_update_task_limit(
            ncp,
            descriptor,
            hw_events,
            events_out,
            DEFAULT_OTA_UPDATE_TASK_LIMIT,
        )
    }

    /// Start the coordinator with a limit on concurrent destination OTA transfer tasks.
    ///
    /// Each destination with an accepted [`crate::ota::Message::Update`] holds one slot for the
    /// complete exchange. Replacing an update for the same destination reuses its task. A limit of
    /// zero rejects every OTA update.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if setting up the actor network fails.
    pub fn start_with_ota_update_task_limit(
        ncp: NcpHandle,
        descriptor: Descriptor,
        hw_events: Receiver<zb_hw::Event>,
        events_out: Sender<Event>,
        ota_update_task_limit: usize,
    ) -> Result<Self, Error> {
        let (ota, ota_inbound) = tokio::sync::mpsc::channel(crate::MPSC_CHANNEL_SIZE);
        let zcl = zcl::Transceiver::spawn(ncp.clone(), events_out.clone(), ota.clone());
        ota::Server::spawn(zcl.clone(), ota_inbound, ota_update_task_limit);
        let zdp = zdp::Transceiver::spawn(ncp.clone(), events_out.clone(), descriptor);
        Mux::spawn(hw_events, events_out, zcl.clone(), zdp.clone());
        Ok(Self { ncp, ota, zcl, zdp })
    }
}
