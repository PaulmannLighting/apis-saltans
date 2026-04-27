use std::collections::BTreeMap;

use log::error;
use tokio::sync::mpsc::{Receiver, Sender};

pub use self::message::Message;
use crate::Event;

mod message;

/// Incoming events demultiplexer.
#[derive(Debug)]
pub struct Demux {
    incoming: Receiver<Message>,
    subscribers: BTreeMap<u8, tokio::sync::oneshot::Sender<Event>>,
    outgoing: Sender<Event>,
}

impl Demux {
    /// Create a new demultiplexer.
    #[must_use]
    pub const fn new(incoming: Receiver<Message>, outgoing: Sender<Event>) -> Self {
        Self {
            incoming,
            subscribers: BTreeMap::new(),
            outgoing,
        }
    }

    /// Run the demultiplexer.
    pub async fn run(mut self) {
        while let Some(message) = self.incoming.recv().await {
            match message {
                Message::Event(event) => {
                    if let Some(event) = self.demux(event) {
                        self.outgoing
                            .send(event)
                            .await
                            .unwrap_or_else(|error| error!("{error:?}"));
                    }
                }
                Message::Subscribe {
                    transaction,
                    response,
                } => {
                    self.subscribers.insert(transaction, response);
                }
            }
        }
    }

    /// Demultiplex an incoming event.
    ///
    /// # Returns
    ///
    /// - `Some(Event)` if the event could not be forwarded to any subscriber.
    /// - `None` if the event was successfully forwarded to a subscriber.
    fn demux(&mut self, event: Event) -> Option<Event> {
        if let Event::MessageReceived {
            src_address,
            aps_frame,
        } = event
        {
            if let Some(subscriber) = self.subscribers.remove(&aps_frame.payload().seq()) {
                subscriber
                    .send(Event::MessageReceived {
                        src_address,
                        aps_frame,
                    })
                    .err()
            } else {
                Some(Event::MessageReceived {
                    src_address,
                    aps_frame,
                })
            }
        } else {
            Some(event)
        }
    }
}
