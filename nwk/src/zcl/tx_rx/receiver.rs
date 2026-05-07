//! ZCL reception layer.
//!
//! Receive ZCL frames.

use zcl::{Cluster, Frame};

use crate::{Command, Error, Event};

/// ZCL reception layer.
pub trait Receiver {
    /// Receive a ZCL frame.
    ///
    /// # Error
    ///
    /// Returns an [`Error`] if receiving the frame fails.
    fn recv(self) -> impl Future<Output = Result<Frame<Cluster>, Error>> + Send;
}

// TODO: Remove panicking paths.
impl Receiver for tokio::sync::oneshot::Receiver<Event> {
    async fn recv(self) -> Result<Frame<Cluster>, Error> {
        let (src_address, (aps_header, command)) = match self.await? {
            Event::MessageReceived {
                src_address,
                aps_frame,
            } => (src_address, aps_frame.into_parts()),
            other => todo!("Handle unexpected event."),
        };

        match command {
            Command::Zcl(frame) => Ok(frame),
            other => todo!("Handle unexpected command type."),
        }
    }
}
