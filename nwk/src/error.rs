use std::fmt::Display;

/// A generic error type for the NWK layer.
#[derive(Debug)]
pub enum Error<T> {
    /// An I/O error occurred.
    Io(std::io::Error),
    /// An implementation-specific error occurred.
    Implementation(T),
    /// An error indicated by a status code.
    Zigbee(zigbee::Error),
    /// An error occurred while sending a message to an actor.
    ActorSend,
    /// An error occurred while receiving a message from an actor.
    ActorReceive,
}

impl<T> Display for Error<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => error.fmt(f),
            Self::Implementation(error) => error.fmt(f),
            Self::Zigbee(error) => error.fmt(f),
            Self::ActorSend => write!(f, "Failed to send message to actor"),
            Self::ActorReceive => write!(f, "Failed to receive message from actor"),
        }
    }
}

impl<T> From<T> for Error<T> {
    fn from(error: T) -> Self {
        Self::Implementation(error)
    }
}

impl<T> std::error::Error for Error<T>
where
    T: std::error::Error + 'static,
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::Implementation(error) => Some(error),
            Self::Zigbee(error) => Some(error),
            Self::ActorSend | Self::ActorReceive => None,
        }
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
