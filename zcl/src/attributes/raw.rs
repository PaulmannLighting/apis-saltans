use zigbee::types::Type;

/// A raw attribute.
pub struct RawAttribute {
    manufacturer_code: Option<u16>,
    cluster: u16,
    id: u16,
    value: Type,
}
