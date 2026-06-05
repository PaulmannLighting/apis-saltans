use std::sync::mpsc::Sender;

use tokio::sync::mpsc::Receiver;

use crate::Event;

/// Trait to implement to translate hardware events into Zigbee events.
pub trait EventTranslator {
    /// The event type issues by the hardware driver to be translated.
    type HardwareEvent;

    /// Create a new event translator.
    fn new(output: Sender<Event>) -> Self;

    /// Run the event translator.
    fn run(input: Receiver<Self::HardwareEvent>) -> impl Future<Output = ()> + Send;
}
