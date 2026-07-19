use std::sync::Arc;

use thiserror::Error;
use tokio::sync::mpsc::error::SendError;
use tokio::sync::oneshot::error::RecvError;

/// A generic error type for Zigbee hardware drivers.
#[derive(Clone, Debug, Error)]
pub enum Error {
    /// An implementation-specific error occurred.
    #[error("{0}")]
    Implementation(#[from] Arc<dyn std::error::Error + Send + Sync>),

    /// An error occurred while sending a message to a driver actor.
    #[error("Failed to send message to driver actor")]
    DriverSend,

    /// An error occurred while receiving a message from a driver actor.
    #[error("Failed to receive message from driver actor")]
    DriverRecv(#[from] RecvError),

    /// An unimplemented feature was invoked.
    #[error("Feature not implemented")]
    NotImplemented,

    /// The NCP did not provide any required local endpoint descriptors.
    #[error("No endpoints provided")]
    NoEndpoints,
}

impl<T> From<SendError<T>> for Error {
    fn from(_: SendError<T>) -> Self {
        Self::DriverSend
    }
}
