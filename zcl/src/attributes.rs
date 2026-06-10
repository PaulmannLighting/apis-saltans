use num_traits::FromPrimitive;
use zigbee::Cluster;
use zigbee::types::Type;

pub use self::errors::{InvalidType, ParseAttributeError};

mod errors;

/// A trait to allow the reading of attributes by their respective IDs in a type-safe manner.
pub trait ReadableAttribute: Cluster + TryFrom<u16, Error = u16> + Into<u16> {
    /// The type of attribute, usually an enum, which is returned from the readable.
    type Attribute: TryFrom<(Self, Type), Error = InvalidType<Self>>;

    /// The manufacturer code of the attribute, if any.
    const MANUFACTURER_CODE: Option<u16> = None;
}
