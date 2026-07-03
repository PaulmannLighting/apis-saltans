use le_stream::{FromLeStream, ToLeStream};

/// Status of an attribute reporting configuration.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
pub struct AttributeStatus {
    status: u8,
    direction: u8,
    attribute_id: u16,
}

impl AttributeStatus {
    /// Creates a new `AttributeStatus`.
    #[must_use]
    pub const fn new(status: u8, direction: u8, attribute_id: u16) -> Self {
        Self {
            status,
            direction,
            attribute_id,
        }
    }

    /// Returns the status.
    #[must_use]
    pub const fn status(&self) -> u8 {
        self.status
    }

    /// Returns the direction.
    #[must_use]
    pub const fn direction(&self) -> u8 {
        self.direction
    }

    /// Returns the attribute ID.
    #[must_use]
    pub const fn attribute_id(&self) -> u16 {
        self.attribute_id
    }
}
