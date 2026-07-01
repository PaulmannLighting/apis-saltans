use std::error::Error;
use std::fmt::{self, Display};

use apis_saltans_aps::Destination;
use apis_saltans_core::Endpoint;

/// Errors that can occur when converting an incoming message to a ZDP frame.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum ParseFrameError {
    /// The source endpoint is invalid (must be 0 for ZDP commands).
    SourceEndpoint(Endpoint),

    /// The destination endpoint is invalid (must be 0 for ZDP commands).
    Destination(Destination),

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
