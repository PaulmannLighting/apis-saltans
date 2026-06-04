use tokio::sync::mpsc::Sender;
use zigbee_hw::Event;

/// Messages received by the multiplexer.
#[derive(Debug)]
pub enum Message {
    /// An event from the hardware layer.
    Event(Event),
    /// Subscribe to any kind of event.
    Subscribe {
        /// The sender to send to.
        sender: Sender<Event>,
    },
}

impl From<Event> for Message {
    fn from(event: Event) -> Self {
        Self::Event(event)
    }
}

impl From<Sender<Event>> for Message {
    fn from(sender: Sender<Event>) -> Self {
        Self::Subscribe { sender }
    }
}
