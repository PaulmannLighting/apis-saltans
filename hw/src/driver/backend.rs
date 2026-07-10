use super::event_translator::EventTranslator;
use crate::Driver;

/// Type-level configuration shared by driver-side backend traits.
pub trait Backend {
    type Driver: Driver;

    /// Hardware-specific event type produced by the driver backend.
    type HardwareEvent: Send + 'static;

    /// Message type consumed by the hardware event translator.
    type Message: From<Self::HardwareEvent> + Send + 'static;

    /// Translator that converts backend messages into crate-level events.
    type EventTranslator: EventTranslator<Message = Self::Message> + 'static;
}
