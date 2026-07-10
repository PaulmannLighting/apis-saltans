#![cfg(feature = "driver-use")]

use apis_saltans_zdp::SimpleDescriptor;
use tokio::sync::mpsc::{Receiver, channel};

use crate::common::{Error, Event, NcpHandle};
use crate::driver::{Backend, EventTranslator, Initialize, bridge};

/// Constructs and prepares a configured hardware backend.
pub trait Builder: Backend + Sized {
    /// Create a driver builder for the endpoints exposed by the coordinator.
    ///
    /// # Errors
    ///
    /// Returns an error if the backend cannot be configured for the supplied endpoint descriptors.
    fn new(endpoints: &[SimpleDescriptor]) -> Result<Self, Error>;

    fn start(
        self,
        hw_events: Receiver<Self::HardwareEvent>,
    ) -> impl Future<
        Output = Result<
            StartedHardware<
                impl Future<Output = ()> + Send + 'static,
                impl Future<Output = ()> + Send + 'static,
            >,
            Error,
        >,
    >
    where
        Self: Initialize,
    {
        let (msg_tx, msg_rx) = channel(hw_events.capacity());
        let (lib_events_tx, events) = channel(hw_events.capacity());
        let br = bridge(hw_events, msg_tx);
        let event_translator = Self::EventTranslator::new(lib_events_tx).run(msg_rx);
        async move {
            let ncp = self.init().await?;
            Ok(StartedHardware {
                ncp,
                events,
                bridge: br,
                translator: event_translator,
            })
        }
    }
}

/// Running hardware support tasks and public handles.
pub struct StartedHardware<Bridge, Translator> {
    /// Handle for sending commands to the NCP actor.
    pub ncp: NcpHandle,

    /// Receiver for translated hardware events.
    pub events: Receiver<Event>,

    /// Future that bridges hardware events into translator messages.
    pub bridge: Bridge,

    /// Future that translates backend messages into crate-level events.
    pub translator: Translator,
}
