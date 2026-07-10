use apis_saltans_zdp::SimpleDescriptor;
use tokio::sync::mpsc::{Receiver, channel};

use super::{EventTranslator, PreparedHardware, bridge};
use crate::Backend;
use crate::common::Error;

/// Constructs and prepares a configured hardware backend.
pub trait Builder: Backend + Sized {
    /// Create a driver builder for the endpoints exposed by the coordinator.
    ///
    /// # Errors
    ///
    /// Returns an error if the backend cannot be configured for the supplied endpoint descriptors.
    fn new(endpoints: &[SimpleDescriptor]) -> Result<Self, Error>;

    /// Prepare the driver startup tasks and event stream.
    fn prepare(
        self,
        hw_events: Receiver<Self::HardwareEvent>,
    ) -> PreparedHardware<
        Self,
        impl Future<Output = ()> + Send + 'static,
        impl Future<Output = ()> + Send + 'static,
    > {
        let (msg_tx, msg_rx) = channel(hw_events.capacity());
        let (lib_events_tx, events) = channel(hw_events.capacity());
        let br = bridge(hw_events, msg_tx);
        let event_translator = Self::EventTranslator::new(lib_events_tx).run(msg_rx);
        PreparedHardware {
            builder: self,
            events,
            bridge: br,
            translator: event_translator,
        }
    }
}
