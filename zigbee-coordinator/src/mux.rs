use tokio::sync::mpsc::error::SendError;
use tokio::sync::mpsc::{Receiver, Sender};
use zigbee_hw::Event;

pub use self::message::Message;
use self::subscribers::Subscribers;

mod message;
mod subscribers;

/// Event multiplexer.
#[derive(Debug, Default)]
pub struct Mux {
    subscribers: Subscribers,
}

impl Mux {
    /// Run the multiplexer.
    pub async fn run(mut self, mut messages: Receiver<Message>) {
        while let Some(message) = messages.recv().await {
            match message {
                Message::Event(event) => self.subscribers.send(&event).await,
                Message::Subscribe { sender } => self.subscribers.add(sender),
            }
        }
    }
}

/// A handle on the multiplexer actor.
pub trait Handle {
    /// Subscribe to an event.
    ///
    /// # Errors
    ///
    /// Returns a [`SendError`] if the multiplexer is no longer running.
    fn subscribe(
        &self,
        sender: Sender<Event>,
    ) -> impl Future<Output = Result<(), SendError<Message>>>;
}

impl Handle for Sender<Message> {
    async fn subscribe(&self, sender: Sender<Event>) -> Result<(), SendError<Message>> {
        self.send(Message::Subscribe { sender }).await
    }
}
