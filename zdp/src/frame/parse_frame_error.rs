use thiserror::Error;
use zb_aps::WeakDestination;

/// Errors that can occur when converting an incoming message to a ZDP frame.
#[derive(Clone, Debug, Eq, Error, PartialEq, Hash)]
pub enum ParseFrameError {
    /// The source endpoint is invalid (must be 0 for ZDP commands).
    #[error("Source endpoint must be 0 for ZDP commands, got {0}")]
    SourceEndpoint(u8),

    /// The destination endpoint is invalid (must be 0 for ZDP commands).
    #[error("Destination endpoint must be 0 for ZDP commands, got {0}")]
    Destination(WeakDestination),

    /// The cluster ID could not be parsed into a ZDP frame.
    #[error("Invalid cluster ID for ZDP frame: {0:#06X}")]
    ClusterId(u16),

    /// The ZDP frame is invalid.
    #[error("Invalid ZDP frame")]
    Invalid,
}
