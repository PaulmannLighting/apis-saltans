use tokio::sync::mpsc::{Receiver, Sender};

pub use self::message::Message;
use crate::{network_manager, transceiver};

mod message;

/// The binding management actor.
#[derive(Debug)]
pub struct Actor {
    transmitter: Sender<transceiver::Message>,
    network_manager: Sender<network_manager::Message>,
}

impl Actor {
    /// Create a new binding management actor.
    #[must_use]
    pub const fn new(
        transmitter: Sender<transceiver::Message>,
        network_manager: Sender<network_manager::Message>,
    ) -> Self {
        Self {
            transmitter,
            network_manager,
        }
    }

    /// Run the binding management actor.
    pub async fn run(self, mut messages: Receiver<Message>) {
        while let Some(message) = messages.recv().await {
            todo!("Handle messages.")
        }
    }
}
