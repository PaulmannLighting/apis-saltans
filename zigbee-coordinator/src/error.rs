//! Coordinator-API errors.

use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    /// Hardware error.
    Hardware(zigbee_hw::Error),

    /// Invalid response type.
    InvalidResponseType,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Hardware(error) => write!(f, "Hardware error: {error}"),
            Self::InvalidResponseType => write!(f, "Invalid response type"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Hardware(error) => Some(error),
            Self::InvalidResponseType => None,
        }
    }
}

impl From<zigbee_hw::Error> for Error {
    fn from(error: zigbee_hw::Error) -> Self {
        Self::Hardware(error)
    }
}
