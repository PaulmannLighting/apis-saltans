use std::collections::BTreeMap;

use tokio::sync::mpsc::Receiver;
use tokio::sync::oneshot::Sender;

pub use self::message::Message;
use crate::Event;

mod message;

/// Incoming events demultiplexer.
#[derive(Debug)]
pub struct Demux {
    incoming: Receiver<Message>,
    subscribers: BTreeMap<u8, Sender<Event>>,
}

impl Demux {
    /// Create a new demultiplexer.
    #[must_use]
    pub const fn new(incoming: Receiver<Message>) -> Self {
        Self {
            incoming,
            subscribers: BTreeMap::new(),
        }
    }

    /// Run the demultiplexer.
    pub async fn run(mut self) {
        while let Some(message) = self.incoming.recv().await {
            match message {
                Message::Event(event) => self.demux(event),
                Message::Subscribe {
                    transaction,
                    response,
                } => {
                    self.subscribers.insert(transaction, response);
                }
            }
        }
    }

    fn demux(&mut self, event: Event) {
        if let Event::MessageReceived {
            src_address,
            aps_frame,
        } = event
            && let Some(subscriber) = self.subscribers.remove(&aps_frame.payload().seq())
        {
            subscriber
                .send(Event::MessageReceived {
                    src_address,
                    aps_frame,
                })
                .expect("Failed to send response.");
        }
    }
}
