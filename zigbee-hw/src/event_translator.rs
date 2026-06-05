use tokio::sync::mpsc::{Receiver, Sender};

use crate::Event;

/// Trait to implement to translate hardware events into Zigbee events.
pub trait EventTranslator {
    /// The input message type to be translated into an [`Event`].
    type Input;

    /// Create a new event translator.
    fn new(output: Sender<Event>) -> Self;

    /// Run the event translator.
    fn run(self, input: Receiver<Self::Input>) -> impl Future<Output = ()> + Send;
}
