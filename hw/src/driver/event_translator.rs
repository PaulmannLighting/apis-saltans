use tokio::sync::mpsc::Receiver;

/// Trait to implement to translate hardware events into Zigbee events.
pub trait EventTranslator {
    /// Hardware-specific message type consumed by this translator.
    type Message;

    /// Run the translator until the input channel closes.
    fn run(self, inbox: Receiver<Self::Message>) -> impl Future<Output = ()> + Send;
}
