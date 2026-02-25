use std::fmt::Display;
use std::sync::Arc;

use tokio::sync::mpsc::error::SendError;
use tokio::sync::oneshot::error::RecvError;

/// A generic error type for the NWK layer.
#[derive(Debug)]
pub enum Error {
    /// An implementation-specific error occurred.
    Implementation(Arc<dyn std::error::Error + Send + Sync>),
    /// An error indicated by a status code.
    Zigbee(zigbee::Error),
    /// An error occurred while sending a message to an actor.
    ActorSend,
    /// An error occurred while receiving a message from an actor.
    ActorReceive,
    /// An unimplemented feature was invoked.
    NotImplemented,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Implementation(error) => error.fmt(f),
            Self::Zigbee(error) => error.fmt(f),
            Self::ActorSend => write!(f, "Failed to send message to actor"),
            Self::ActorReceive => write!(f, "Failed to receive message from actor"),
            Self::NotImplemented => write!(f, "Feature not implemented"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Implementation(error) => Some(&**error),
            Self::Zigbee(error) => Some(error),
            Self::ActorSend | Self::ActorReceive | Self::NotImplemented => None,
        }
    }
}

impl From<RecvError> for Error {
    fn from(_: RecvError) -> Self {
        Self::ActorReceive
    }
}

impl<T> From<SendError<T>> for Error {
    fn from(_: SendError<T>) -> Self {
        Self::ActorSend
    }
}

pub mod zigbee {
    use std::fmt::Display;

    /// A Zigbee-protocol error.
    ///
    /// TODO: Implement and move to appropriate library.
    #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
    pub enum Error {}

    impl Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Zigbee error")
        }
    }

    impl std::error::Error for Error {}
}
