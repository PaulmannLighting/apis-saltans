use tokio::sync::mpsc::{Receiver, Sender};

use crate::Event;

/// Trait to implement to translate hardware events into Zigbee events.
pub trait EventTranslator {
    /// Hardware-specific message type consumed by this translator.
    type Message;

    /// Create a translator that emits common hardware events to `output`.
    fn new(output: Sender<Event>) -> Self;

    /// Run the translator until the input channel closes.
    fn run(self, inbox: Receiver<Self::Message>) -> impl Future<Output = ()> + Send;
}
