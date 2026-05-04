use std::fmt::Display;

use tokio::sync::mpsc::error::SendError;
use tokio::sync::oneshot::error::RecvError;

use crate::demux::Message;

/// Error that can occur when sending to or receiving from a demultiplexer.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Error {
    /// Subscribing failed.
    Subscribe,

    /// Receiving failed.
    Receive,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Subscribe => write!(f, "Subscribe error"),
            Self::Receive => write!(f, "Receive error"),
        }
    }
}

impl std::error::Error for Error {}

impl From<RecvError> for Error {
    fn from(_: RecvError) -> Self {
        Self::Receive
    }
}

impl From<SendError<Message>> for Error {
    fn from(_: SendError<Message>) -> Self {
        Self::Receive
    }
}

impl From<Error> for crate::Error {
    fn from(error: Error) -> Self {
        match error {
            Error::Subscribe => Self::ActorSend,
            Error::Receive => Self::ActorReceive,
        }
    }
}
