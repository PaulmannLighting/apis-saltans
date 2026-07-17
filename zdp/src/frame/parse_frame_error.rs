use std::error::Error;
use std::fmt::{self, Display};

use zb_aps::WeakDestination;

/// Errors that can occur when converting an incoming message to a ZDP frame.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum ParseFrameError {
    /// The source endpoint is invalid (must be 0 for ZDP commands).
    SourceEndpoint(u8),

    /// The destination endpoint is invalid (must be 0 for ZDP commands).
    Destination(WeakDestination),

    /// The cluster ID could not be parsed into a ZDP frame.
    ClusterId(u16),

    /// The ZDP frame is invalid.
    Invalid,
}

impl Display for ParseFrameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SourceEndpoint(endpoint) => {
                write!(
                    f,
                    "Source endpoint must be 0 for ZDP commands, got {endpoint}"
                )
            }
            Self::Destination(destination) => {
                write!(
                    f,
                    "Destination endpoint must be 0 for ZDP commands, got {destination}",
                )
            }
            Self::ClusterId(cluster_id) => {
                write!(f, "Invalid cluster ID for ZDP frame: {cluster_id:#06X}")
            }
            Self::Invalid => write!(f, "Invalid ZDP frame"),
        }
    }
}

impl Error for ParseFrameError {}
