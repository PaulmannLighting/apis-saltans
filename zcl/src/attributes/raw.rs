use zigbee::types::Type;

/// A raw attribute.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct RawAttribute {
    cluster: u16,
    id: u16,
    value: Type,
    manufacturer_code: Option<u16>,
}

impl RawAttribute {
    /// Create a new `RawAttribute`.
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

    /// Return the payload.
    #[must_use]
    pub const fn value(&self) -> &Type {
        &self.value
    }

    /// Return the manufacturer code, if any.
    #[must_use]
    pub const fn manufacturer_code(&self) -> Option<u16> {
        self.manufacturer_code
    }

    /// Return `true` iff this is a manufacturer-specific attribute.
    #[must_use]
    pub const fn is_manufacturer_specific(&self) -> bool {
        self.manufacturer_code.is_some()
    }
}
