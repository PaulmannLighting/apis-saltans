use core::error::Error;
use core::fmt::{Debug, Display};

use zigbee::types::Type;

/// An error that occurs when parsing an attribute fails.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum ParseAttributeError<T> {
    /// The attribute ID is invalid.
    InvalidId(u16),

    /// The attribute type is invalid for this ID.
    InvalidType(InvalidType<T>),
}

impl<T> Display for ParseAttributeError<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidId(id) => write!(f, "Invalid attribute ID: {id}"),
            Self::InvalidType(error) => write!(f, "{error}"),
        }
    }
}

impl<T> Error for ParseAttributeError<T>
where
    T: Debug + Display + 'static,
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidId(_) => None,
            Self::InvalidType(error) => Some(error),
        }
    }
}

impl<T> From<InvalidType<T>> for ParseAttributeError<T> {
    fn from(error: InvalidType<T>) -> Self {
        Self::InvalidType(error)
    }
}

/// The data type is invalid for the given attribute ID.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct InvalidType<T> {
    id: T,
    typ: Type,
}

impl<T> InvalidType<T> {
    /// Create a new invalid type error.
    #[must_use]
    pub const fn new(id: T, typ: Type) -> Self {
        Self { id, typ }
    }
}

impl<T> Display for InvalidType<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Invalid type {:?} for attribute with id {}",
            self.typ, self.id
        )
    }
}

impl<T> Error for InvalidType<T> where T: Debug + Display {}
