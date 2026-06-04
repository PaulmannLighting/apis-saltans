use tokio::sync::mpsc::Receiver;

pub use self::message::Message;
use self::subscribers::Subscribers;

mod message;
mod subscribers;

/// Event multiplexer
#[derive(Debug, Default)]
pub struct Mux {
    subscribers: Subscribers,
}

impl Mux {
    pub async fn run(mut self, mut messages: Receiver<Message>) {
        while let Some(message) = messages.recv().await {
            match message {
                Message::Event(event) => self.subscribers.send(&event).await,
                Message::Subscribe { sender } => self.subscribers.add(sender),
            }
        }
    }
}
