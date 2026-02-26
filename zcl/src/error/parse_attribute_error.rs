use core::error::Error;
use core::fmt;
use core::fmt::{Display, Formatter};

use zigbee::types::Type;

/// Error when parsing an attribute enum from an ID and `Type`.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum ParseAttributeError {
    /// The attribute ID is invalid for this attribute enum.
    InvalidId(u16),
    /// The data type is invalid for this attribute variant.
    InvalidType(Type),
}

impl Display for ParseAttributeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidId(id) => write!(f, "Invalid attribute ID: {id:#06X}"),
            Self::InvalidType(ty) => {
                write!(f, "Invalid attribute type: {:#04X}", ty.discriminant())
            }
        }
    }
}

impl Error for ParseAttributeError {}
