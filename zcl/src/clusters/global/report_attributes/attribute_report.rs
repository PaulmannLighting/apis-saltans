use le_stream::{FromLeStream, ToLeStream};
use zigbee::types::Type;

/// Attribute report.
#[derive(Clone, Debug, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
pub struct AttributeReport {
    attribute_id: u16,
    data: Type,
}

impl AttributeReport {
    /// Create a new `AttributeReport`.
    #[must_use]
    pub const fn new(attribute_id: u16, data: Type) -> Self {
        Self { attribute_id, data }
    }

    /// Return the attribute ID.
    #[must_use]
    pub const fn attribute_id(&self) -> u16 {
        self.attribute_id
    }

    /// Return the data.
    #[must_use]
    pub const fn data(&self) -> &Type {
        &self.data
    }
}
