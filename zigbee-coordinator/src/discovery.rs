use tokio::sync::mpsc::{Receiver, Sender};

pub use self::message::Message;
use crate::{binding, transceiver};

mod message;

/// The device discovery actor.
#[derive(Debug)]
pub struct Actor {
    transmitter: Sender<transceiver::Message>,
    binding_manager: Sender<binding::Message>,
}

impl Actor {
    /// Create a new discovery actor.
    #[must_use]
    pub const fn new(
        transmitter: Sender<transceiver::Message>,
        binding_manager: Sender<binding::Message>,
    ) -> Self {
        Self {
            transmitter,
            binding_manager,
        }
    }

    /// Run the discovery actor.
    pub async fn run(self, mut messages: Receiver<Message>) {
        while let Some(message) = messages.recv().await {
            todo!("Handle events.")
        }
    }
}
