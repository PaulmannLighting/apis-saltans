use std::fmt::Debug;

use tokio::sync::mpsc::{Receiver, Sender};
use zb_hw::{Error, NcpHandle};
use zb_zdp::SimpleDescriptor;

use crate::mux::Mux;
use crate::{Event, zcl, zdp};

/// External Zigbee API struct.
#[derive(Clone, Debug)]
pub struct Coordinator {
    pub(crate) ncp: NcpHandle,
    pub(crate) zcl: Sender<zcl::Message>,
    pub(crate) zdp: Sender<zdp::Message>,
}

impl Coordinator {
    /// Start the coordinator on the given hardware.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if setting up the actor network fails.
    pub fn start(
        ncp: NcpHandle,
        hw_events: Receiver<zb_hw::Event>,
        events_out: Sender<Event>,
        endpoints: &[SimpleDescriptor],
    ) -> Result<Self, Error> {
        let zcl = zcl::Transceiver::spawn(ncp.clone(), events_out.clone());
        let zdp = zdp::Transceiver::spawn(ncp.clone(), events_out.clone(), endpoints);
        Mux::spawn(ncp.clone(), hw_events, events_out, zcl.clone(), zdp.clone());
        Ok(Self { ncp, zcl, zdp })
    }
}
