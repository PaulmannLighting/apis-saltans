use le_stream::{FromLeStream, ToLeStream};

/// Extended attribute information record of a Discover Attributes Extended response.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
pub struct AttributeInformation {
    id: u16,
    data_type: u8,
    access_control: u8,
}

impl AttributeInformation {
    /// Create an extended attribute information record.
    #[must_use]
    pub const fn new(
        attribute_id: u16,
        attribute_data_type: u8,
        attribute_access_control: u8,
    ) -> Self {
        Self {
            id: attribute_id,
            data_type: attribute_data_type,
            access_control: attribute_access_control,
        }
    }

    /// Return the attribute identifier.
    #[must_use]
    pub const fn attribute_id(&self) -> u16 {
        self.id
    }

    /// Return the attribute data type identifier.
    #[must_use]
    pub const fn attribute_data_type(&self) -> u8 {
        self.data_type
    }

    /// Return the attribute access-control bitmask.
    #[must_use]
    pub const fn attribute_access_control(&self) -> u8 {
        self.access_control
    }
}
