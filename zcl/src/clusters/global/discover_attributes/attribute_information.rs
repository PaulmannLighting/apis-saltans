use le_stream::{FromLeStream, ToLeStream};

/// Attribute information record of a Discover Attributes response.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
pub struct AttributeInformation {
    attribute_id: u16,
    attribute_data_type: u8,
}

impl AttributeInformation {
    /// Create an attribute information record.
    #[must_use]
    pub const fn new(attribute_id: u16, attribute_data_type: u8) -> Self {
        Self {
            attribute_id,
            attribute_data_type,
        }
    }

    /// Return the attribute identifier.
    #[must_use]
    pub const fn attribute_id(&self) -> u16 {
        self.attribute_id
    }

    /// Return the attribute data type identifier.
    #[must_use]
    pub const fn attribute_data_type(&self) -> u8 {
        self.attribute_data_type
    }
}
