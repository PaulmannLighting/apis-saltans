use std::fmt::Debug;

use tokio::sync::mpsc::{Receiver, Sender};
use zb_core::node::Descriptor;
use zb_hw::{Error, NcpHandle};

use crate::event_sink::EventSink;
use crate::mux::Mux;
use crate::{DEFAULT_OTA_UPDATE_TASK_LIMIT, Event, aps, ota, zcl, zdp};

/// External Zigbee API struct.
#[derive(Clone, Debug)]
pub struct Coordinator {
    pub(crate) ncp: NcpHandle,
    pub(crate) ota: Sender<ota::Message>,
    pub(crate) zcl: Sender<zcl::Message>,
    pub(crate) zdp: Sender<zdp::Message>,
}

impl Coordinator {
    /// Start the coordinator on the given hardware.
    ///
    /// Local endpoint descriptors are obtained through [`NcpHandle::get_endpoints`] when needed;
    /// callers do not supply them during startup. The hardware event timestamp and link-key
    /// device-pair handle types may be selected by the backend.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if setting up the actor network fails.
    pub fn start<T, K>(
        ncp: NcpHandle,
        descriptor: Descriptor,
        hw_events: Receiver<zb_hw::Event<T, K>>,
        events_out: Sender<Event>,
    ) -> Result<Self, Error>
    where
        T: Send + 'static,
        K: Send + 'static,
    {
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
    /// zero rejects every OTA update. The hardware event timestamp and link-key device-pair handle
    /// types may be selected by the backend.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if setting up the actor network fails.
    pub fn start_with_ota_update_task_limit<T, K>(
        ncp: NcpHandle,
        descriptor: Descriptor,
        hw_events: Receiver<zb_hw::Event<T, K>>,
        events_out: Sender<Event>,
        ota_update_task_limit: usize,
    ) -> Result<Self, Error>
    where
        T: Send + 'static,
        K: Send + 'static,
    {
        let events = EventSink::new(events_out);
        let aps = aps::Transceiver::spawn(ncp.clone());
        let zcl = zcl::Transceiver::spawn(aps.clone(), events.clone());
        let ota = ota::Server::spawn(ncp.clone(), zcl.clone(), ota_update_task_limit);
        let zdp = zdp::Transceiver::spawn(ncp.clone(), aps.clone(), events.clone(), descriptor);
        Mux::spawn(
            hw_events,
            events,
            aps,
            ota.clone(),
            zcl.clone(),
            zdp.clone(),
        );
        Ok(Self { ncp, ota, zcl, zdp })
    }
}
