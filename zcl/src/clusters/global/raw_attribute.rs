//! A raw read attribute.

use zigbee::types::Type;

/// A raw read attribute.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct RawAttribute {
    cluster: u16,
    id: u16,
    value: Type,
    manufacturer_code: Option<u16>,
}

impl RawAttribute {
    /// Create a new attribute.
    #[must_use]
    pub const fn new(cluster: u16, id: u16, value: Type, manufacturer_code: Option<u16>) -> Self {
        Self {
            cluster,
            id,
            value,
            manufacturer_code,
        }
    }

    /// Return the cluster ID.
    #[must_use]
    pub const fn cluster(&self) -> u16 {
        self.cluster
    }

    /// Return the attribute ID.
    #[must_use]
    pub const fn id(&self) -> u16 {
        self.id
    }

    /// Return the raw value.
    #[must_use]
    pub const fn value(&self) -> &Type {
        &self.value
    }

    /// Return the manufacturer code.
    #[must_use]
    pub const fn manufacturer_code(&self) -> Option<u16> {
        self.manufacturer_code
    }
}

impl From<RawAttribute> for Type {
    fn from(raw: RawAttribute) -> Self {
        raw.value
    }
}
