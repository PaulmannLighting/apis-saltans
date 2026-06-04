use tokio::sync::mpsc::{Receiver, Sender};
use zigbee_hw::Event;

use crate::{binding, transmitter};

/// The device discovery actor.
#[derive(Debug)]
pub struct Actor {
    transmitter: Sender<transmitter::Message>,
    binding_manager: Sender<binding::Message>,
}

impl Actor {
    /// Create a new discovery actor.
    #[must_use]
    pub const fn new(
        transmitter: Sender<transmitter::Message>,
        binding_manager: Sender<binding::Message>,
    ) -> Self {
        Self {
            transmitter,
            binding_manager,
        }
    }

    /// Run the discovery actor.
    pub async fn run(self, mut events: Receiver<Event>) {
        while let Some(event) = events.recv().await {
            todo!("Handle events.")
        }
    }
}
