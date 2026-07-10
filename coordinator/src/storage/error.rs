use std::fmt::Display;

use tokio::sync::mpsc::error::SendError;
use tokio::sync::oneshot::error::RecvError;

/// Storage API errors.
#[derive(Debug)]
pub enum Error {
    /// An error occurred while sending to the storage server.
    Send,

    /// An error occurred while receiving from the storage server.
    Receive(RecvError),

    /// An I/O error occurred.
    Io(std::io::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Send => write!(f, "Send error"),
            Self::Receive(e) => e.fmt(f),
            Self::Io(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Send => None,
            Self::Receive(error) => Some(error),
            Self::Io(error) => Some(error),
        }
    }
}

impl<T> From<SendError<T>> for Error {
    fn from(_: SendError<T>) -> Self {
        Self::Send
    }
}

impl From<RecvError> for Error {
    fn from(error: RecvError) -> Self {
        Self::Receive(error)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}
