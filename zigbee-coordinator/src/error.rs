//! Coordinator-API errors.

use std::fmt::Display;

use tokio::sync::mpsc::error::SendError;
use tokio::sync::oneshot::error::RecvError;

#[derive(Debug)]
pub enum Error {
    /// Hardware error.
    Hardware(zigbee_hw::Error),

    /// Transmission through the channel failed.
    SendError,

    /// Receiving of response failed.
    ReceiveError(RecvError),

    /// Invalid response type.
    InvalidResponseType,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Hardware(error) => write!(f, "Hardware error: {error}"),
            Self::SendError => write!(f, "Sending failed"),
            Self::ReceiveError(error) => write!(f, "Receiving failed: {error}"),
            Self::InvalidResponseType => write!(f, "Invalid response type"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Hardware(error) => Some(error),
            Self::ReceiveError(error) => Some(error),
            Self::SendError | Self::InvalidResponseType => None,
        }
    }
}

impl From<zigbee_hw::Error> for Error {
    fn from(error: zigbee_hw::Error) -> Self {
        Self::Hardware(error)
    }
}

impl<T> From<SendError<T>> for Error {
    fn from(_: SendError<T>) -> Self {
        Self::SendError
    }
}

impl From<RecvError> for Error {
    fn from(error: RecvError) -> Self {
        Self::ReceiveError(error)
    }
}
