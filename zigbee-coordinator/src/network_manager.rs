use tokio::sync::mpsc::Receiver;

pub use self::device::Device;
pub use self::message::Message;

mod device;
mod message;

/// The network management actor.
#[derive(Debug, Default)]
pub struct Actor {}

impl Actor {
    /// Run the actor.
    pub async fn run(self, mut messages: Receiver<Message>) {
        while let Some(message) = messages.recv().await {
            todo!("Handle messages.")
        }
    }
}
