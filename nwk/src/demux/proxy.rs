use tokio::sync::mpsc::Sender;
use tokio::sync::mpsc::error::SendError;
use tokio::sync::oneshot::{Receiver, channel};

use crate::Event;
use crate::demux::Message;

/// Proxy to access the demultiplexer.
pub trait Proxy {
    /// Subscribe to the demux.
    ///
    /// # Errors
    ///
    /// Returns a [`SendError`] if the message could not be sent to the actor.
    fn subscribe(
        &self,
        seq: u8,
    ) -> impl Future<Output = Result<Receiver<Event>, SendError<Message>>>;
}

impl Proxy for Sender<Message> {
    async fn subscribe(&self, seq: u8) -> Result<Receiver<Event>, SendError<Message>> {
        let (tx, rx) = channel();
        self.send(Message::subscribe(seq, tx)).await.map(|()| rx)
    }
}
