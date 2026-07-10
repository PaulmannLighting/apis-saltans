use apis_saltans_zdp::SimpleDescriptor;
use tokio::sync::mpsc::{Receiver, channel};

use super::{EventTranslator, PreparedHardware, bridge};
use crate::common::Error;

/// Constructs and wires a hardware driver from endpoint descriptors.
pub trait Builder: Sized {
    /// Hardware-specific event type produced by the driver backend.
    type HardwareEvent: Send + 'static;

    /// Message type consumed by the hardware event translator.
    type Message: From<Self::HardwareEvent> + Send + 'static;

    /// Translator that converts backend messages into crate-level events.
    type EventTranslator: EventTranslator<Message = Self::Message> + 'static;

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
