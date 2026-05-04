use std::fmt::Display;

use tokio::sync::mpsc::error::SendError;
use tokio::sync::oneshot::error::RecvError;

use crate::demux::Message;

/// Error that can occur when sending to or receiving from a demultiplexer.
#[derive(Debug)]
pub enum Error {
    /// Subscribing failed.
    Subscribe(SendError<Message>),

    /// Receiving failed.
    Receive(RecvError),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Subscribe(error) => write!(f, "{error}"),
            Self::Receive(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Subscribe(error) => Some(error),
            Self::Receive(error) => Some(error),
        }
    }
}

impl From<RecvError> for Error {
    fn from(error: RecvError) -> Self {
        Self::Receive(error)
    }
}

impl From<SendError<Message>> for Error {
    fn from(error: SendError<Message>) -> Self {
        Self::Subscribe(error)
    }
}

impl From<Error> for crate::Error {
    fn from(error: Error) -> Self {
        match error {
            Error::Subscribe(_) => Self::ActorSend,
            Error::Receive(_) => Self::ActorReceive,
        }
    }
}
