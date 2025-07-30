use core::error::Error;
use core::fmt::{Debug, Display};

/// Error when parsing a `DateCode`.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ParseError {
    /// The date within the string is invalid.
    InvalidDate(chrono::ParseError),
    /// The custom part of the date code is too long.
    CustomPartTooLong,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidDate(error) => write!(f, "{error}"),
            Self::CustomPartTooLong => {
                write!(f, "Custom part of date code is too long.")
            }
        }
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidDate(error) => Some(error),
            Self::CustomPartTooLong => None,
        }
    }
}

impl From<chrono::ParseError> for ParseError {
    fn from(error: chrono::ParseError) -> Self {
        Self::InvalidDate(error)
    }
}

impl From<()> for ParseError {
    fn from((): ()) -> Self {
        Self::CustomPartTooLong
    }
}
