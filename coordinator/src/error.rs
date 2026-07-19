//! Coordinator-API errors.

use std::fmt::{self, Display};
use std::time::Duration;

use thiserror::Error as ThisError;
use tokio::sync::mpsc::error::SendError;
use tokio::sync::oneshot::error::RecvError;
use tokio::time::error::Elapsed;
use zb_core::IeeeAddress;

pub use self::optional::Optional;
pub use self::status_ext::StatusExt;

mod optional;
mod status_ext;

/// Errors that can occur in the coordinator-API.
#[derive(Debug, ThisError)]
pub enum Error {
    /// Hardware error.
    #[error("Hardware error: {0}")]
    Hardware(#[from] zb_hw::Error),

    /// Transmission through the channel failed.
    #[error("Sending failed")]
    SendError,

    /// Receiving of response failed.
    #[error("Receiving failed: {0}")]
    ReceiveError(#[from] RecvError),

    /// Invalid response type.
    #[error("Invalid response type: {0}")]
    InvalidResponseType(String),

    /// Unknown device.
    #[error("Unknown device: {0}")]
    UnknownDevice(IeeeAddress),

    /// Invalid application endpoint.
    #[error("Invalid application endpoint: {0:#04X}")]
    InvalidApplicationEndpoint(u8),

    /// Invalid rate.
    #[error("Invalid dimming rate: {0:?}")]
    DurationOutOfBounds(Duration),

    /// ZCL status error, preserving unknown raw status bytes.
    #[error("ZCL error: {}", display_status(.0))]
    Zcl(Result<zb_zcl::Status, u8>),

    /// ZDP status error, preserving unknown raw status bytes.
    #[error("ZDP error: {}", display_status(.0))]
    Zdp(Result<zb_zdp::Status, u8>),

    /// A request exceeded its allotted response time.
    #[error("Timeout: {0:?}")]
    Timeout(#[from] Elapsed),
}

impl<T> From<SendError<T>> for Error {
    fn from(_: SendError<T>) -> Self {
        Self::SendError
    }
}

impl From<Result<zb_zcl::Status, u8>> for Error {
    fn from(status: Result<zb_zcl::Status, u8>) -> Self {
        Self::Zcl(status)
    }
}

impl From<Result<zb_zdp::Status, u8>> for Error {
    fn from(error: Result<zb_zdp::Status, u8>) -> Self {
        Self::Zdp(error)
    }
}

fn display_status<T>(status: &Result<T, u8>) -> impl Display + '_
where
    T: Display,
{
    fmt::from_fn(|f| match status {
        Ok(status) => status.fmt(f),
        Err(raw) => write!(f, "{raw:#04x}"),
    })
}
