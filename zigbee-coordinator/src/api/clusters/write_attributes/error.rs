use std::fmt::Display;

/// Errors that can occur when writing attributes.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Error {
    /// The attribute could not be written.
    WriteFailed(u16),
    /// There was no response for the written attribute.
    NoResponse(u16),
    /// A response for writing of an attribute was received, but the attribute was not written.
    NotRequested(u16),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WriteFailed(id) => write!(f, "Failed to write attribute: {id:#06X}"),
            Self::NoResponse(id) => write!(f, "No response for attribute: {id:#06X}"),
            Self::NotRequested(id) => {
                write!(f, "Attribute {id:#06X} was not requested to be written.")
            }
        }
    }
}

impl std::error::Error for Error {}
