use std::error::Error;
use std::fmt::{Debug, Display};

/// Error when parsing a `DateCode`.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ParseError {
    /// The date within the string is invalid.
    InvalidDate(chrono::ParseError),
    /// The custom part of the date code is too long.
    CustomPartTooLong,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::InvalidDate(error) => write!(f, "{error}"),
            ParseError::CustomPartTooLong => {
                write!(f, "Custom part of date code is too long.")
            }
        }
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ParseError::InvalidDate(error) => Some(error),
            ParseError::CustomPartTooLong => None,
        }
    }
}

impl From<chrono::ParseError> for ParseError {
    fn from(error: chrono::ParseError) -> Self {
        ParseError::InvalidDate(error)
    }
}

impl From<()> for ParseError {
    fn from(_: ()) -> Self {
        ParseError::CustomPartTooLong
    }
}
