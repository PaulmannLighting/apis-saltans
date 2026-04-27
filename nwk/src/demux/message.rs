use tokio::sync::oneshot::Sender;

use crate::Event;

/// Message to send to the demultiplexer.
pub enum Message {
    /// An incoming event.
    Event(Event),
    /// A subscription request.
    Subscribe {
        /// The transaction number.
        transaction: u8,
        /// The response channel.
        response: Sender<Event>,
    },
}

impl Message {
    /// Create a new subscription request.
    #[must_use]
    pub const fn subscribe(transaction: u8, response: Sender<Event>) -> Self {
        Self::Subscribe {
            transaction,
            response,
        }
    }
}

impl From<Event> for Message {
    fn from(event: Event) -> Self {
        Self::Event(event)
    }
}
