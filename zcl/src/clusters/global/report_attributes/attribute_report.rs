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
    ///
    /// # Safety
    ///
    /// The caller must ensure that the attribute and data type match.
    #[must_use]
    #[expect(unsafe_code)]
    pub const unsafe fn new(attribute_id: u16, data: Type) -> Self {
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

    /// Return the data.
    #[must_use]
    pub fn into_data(self) -> Type {
        self.data
    }

    /// Returns the attribute ID.
    #[must_use]
    pub fn into_parts(self) -> (u16, Type) {
        (self.attribute_id, self.data)
    }
}
