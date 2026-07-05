use apis_saltans_core::Cluster;
use apis_saltans_core::types::Type;

pub use self::errors::{InvalidType, ParseAttributeError};
use crate::global::write_attributes::Record;

mod errors;

/// A trait to allow the reading of attributes by their respective IDs in a type-safe manner.
pub trait Readable: Cluster + TryFrom<u16, Error = u16> + Into<u16> {
    /// The manufacturer code of the attribute, if any.
    const MANUFACTURER_CODE: Option<u16> = None;

    /// The type of attribute, usually an enum, which is returned from the readable.
    type Attribute: TryFrom<(Self, Type), Error = InvalidType<Self>>;
}

/// A trait to allow the writing of attribute values in a type-safe manner.
pub trait Writable: Cluster + Into<Record> {
    /// The manufacturer code of the attribute, if any.
    const MANUFACTURER_CODE: Option<u16> = None;

    /// The ID of the attribute.
    fn id(&self) -> u16;
}
