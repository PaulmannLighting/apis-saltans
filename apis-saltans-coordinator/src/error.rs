//! Coordinator-API errors.

use std::fmt::Display;
use std::time::Duration;

use macaddr::MacAddr8;
use tokio::sync::mpsc::error::SendError;
use tokio::sync::oneshot::error::RecvError;
use tokio::time::error::Elapsed;

/// Errors that can occur in the coordinator-API.
#[derive(Debug)]
pub enum Error {
    /// Hardware error.
    Hardware(apis_saltans_hw::Error),

    /// Transmission through the channel failed.
    SendError,

    /// Receiving of response failed.
    ReceiveError(RecvError),

    /// Timeout while waiting for a response.
    Timeout(Elapsed),

    /// Invalid response type.
    InvalidResponseType(String),

    /// Unknown device.
    UnknownDevice(MacAddr8),

    /// Invalid application endpoint.
    InvalidApplicationEndpoint(u8),

    /// Invalid rate.
    DurationOutOfBounds(Duration),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Hardware(error) => write!(f, "Hardware error: {error}"),
            Self::SendError => write!(f, "Sending failed"),
            Self::ReceiveError(error) => write!(f, "Receiving failed: {error}"),
            Self::Timeout(error) => write!(f, "Timeout: {error}"),
            Self::InvalidResponseType(error) => write!(f, "Invalid response type: {error}"),
            Self::UnknownDevice(address) => write!(f, "Unknown device: {address}"),
            Self::InvalidApplicationEndpoint(endpoint) => {
                write!(f, "Invalid application endpoint: {endpoint:#04X}")
            }
            Self::DurationOutOfBounds(rate) => write!(f, "Invalid dimming rate: {rate:?}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Hardware(error) => Some(error),
            Self::ReceiveError(error) => Some(error),
            Self::Timeout(error) => Some(error),
            Self::SendError
            | Self::InvalidResponseType(_)
            | Self::UnknownDevice(_)
            | Self::InvalidApplicationEndpoint(_)
            | Self::DurationOutOfBounds(_) => None,
        }
    }
}

impl From<apis_saltans_hw::Error> for Error {
    fn from(error: apis_saltans_hw::Error) -> Self {
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

impl From<Elapsed> for Error {
    fn from(error: Elapsed) -> Self {
        Self::Timeout(error)
    }
}
