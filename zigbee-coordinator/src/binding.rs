use tokio::sync::mpsc::Receiver;

pub use self::message::Message;

mod message;

/// The binding management actor.
#[derive(Debug, Default)]
pub struct Actor {}

impl Actor {
    pub async fn run(self, mut messages: Receiver<Message>) {
        while let Some(message) = messages.recv().await {
            todo!("Handle messages.")
        }
    }
}
