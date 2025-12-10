use core::error::Error;
use core::fmt;
use core::fmt::Display;

/// Frame parsing error.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ParseFrameError {
    /// The ZCL frame header is invalid.
    MissingHeader,
    /// Invalid cluster ID.
    InvalidClusterId(u16),
    /// Invalid command ID.
    InvalidCommandId(u8),
    /// The amount of bytes of the payload is insufficient.
    InsufficientPayload,
}

impl Display for ParseFrameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingHeader => write!(f, "Missing ZCL frame header"),
            Self::InvalidClusterId(cluster_id) => {
                write!(f, "Invalid cluster ID: {cluster_id}")
            }
            Self::InvalidCommandId(command_id) => {
                write!(f, "Invalid command ID: {command_id}")
            }
            Self::InsufficientPayload => {
                write!(f, "Insufficient payload bytes")
            }
        }
    }
}

impl Error for ParseFrameError {}
