use log::trace;
use tokio::sync::mpsc::error::SendError;
use tokio::sync::mpsc::{Receiver, Sender};
use zigbee_hw::Event;

pub use self::message::Message;

mod message;

/// Event multiplexer.
#[derive(Debug, Default)]
pub struct Mux {
    subscribers: Vec<Sender<Event>>,
}

impl Mux {
    /// Run the multiplexer.
    pub async fn run(mut self, mut messages: Receiver<Message>) {
        while let Some(message) = messages.recv().await {
            match message {
                Message::Event(event) => {
                    for subscriber in self.subscribers.drain(..) {
                        if let Err(error) = subscriber.send(event.clone()).await {
                            trace!("Subscriber went away: {error}");
                        }
                    }

                    self.subscribers
                        .retain(|subscriber| !subscriber.is_closed());
                }
                Message::Subscribe { sender } => self.subscribers.push(sender),
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
