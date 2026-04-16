use std::fmt::Display;
use std::num::TryFromIntError;

#[derive(Debug)]
pub enum Error {
    /// A zigbee-nwk error occurred.
    ZigbeeNwk(crate::Error),
    /// The passed duration is invalid.
    InvalidDuration(TryFromIntError),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ZigbeeNwk(error) => write!(f, "{error}"),
            Self::InvalidDuration(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ZigbeeNwk(error) => Some(error),
            Self::InvalidDuration(error) => Some(error),
        }
    }
}

impl From<crate::Error> for Error {
    fn from(error: crate::Error) -> Self {
        Self::ZigbeeNwk(error)
    }
}

impl From<TryFromIntError> for Error {
    fn from(error: TryFromIntError) -> Self {
        Self::InvalidDuration(error)
    }
}
