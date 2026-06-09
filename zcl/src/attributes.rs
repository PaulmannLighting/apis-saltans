use zigbee::Cluster;
use zigbee::types::Type;

pub mod readable;

/// The result of parsing an attribute value.
pub type ParseResult<T> = Result<
    <T as ReadableAttribute>::Attribute,
    <<T as ReadableAttribute>::Attribute as TryFrom<(u16, Type)>>::Error,
>;

/// A trait to allow the reading of attributes by their respective IDs in a type-safe manner.
pub trait ReadableAttribute: Copy + Cluster + Into<u16> {
    /// The type of attribute, usually an enum, which is returned from the readable.
    type Attribute: TryFrom<(u16, Type)>;

    /// The manufacturer code of the attribute, if any.
    const MANUFACTURER_CODE: Option<u16> = None;
}
