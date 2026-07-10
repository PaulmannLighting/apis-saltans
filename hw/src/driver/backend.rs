use super::event_translator::EventTranslator;

/// Type-level configuration shared by driver-side backend traits.
pub trait Backend {
    /// Hardware-specific event type produced by the driver backend.
    type HardwareEvent: Send + 'static;

    /// Message type consumed by the hardware event translator.
    type Message: From<Self::HardwareEvent> + Send + 'static;

    /// Translator that converts backend messages into crate-level events.
    type EventTranslator: EventTranslator<Message = Self::Message> + 'static;
}
