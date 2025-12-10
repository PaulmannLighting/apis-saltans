use core::error::Error;
use core::fmt;
use core::fmt::Display;

/// An enumeration of possible errors that can occur while parsing a frame.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ParseFrameError {
    /// The sequence number is missing.
    MissingSeq,
    /// The cluster ID is invalid.
    InvalidCluster(u16),
    /// There are not enough bytes to parse the payload.
    InsufficientPayload,
}

impl Display for ParseFrameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseFrameError::MissingSeq => write!(f, "Missing sequence number"),
            ParseFrameError::InvalidCluster(id) => write!(f, "Invalid cluster ID: {}", id),
            ParseFrameError::InsufficientPayload => write!(f, "Insufficient payload bytes"),
        }
    }
}

impl Error for ParseFrameError {}
