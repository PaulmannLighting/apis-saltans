use le_stream::{FromLeStream, ToLeStream};

/// Attribute record of a Read Reporting Configuration command.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
pub struct Record {
    direction: u8,
    attribute_id: u16,
}

impl Record {
    /// Create a reporting-configuration request record.
    #[must_use]
    pub const fn new(direction: u8, attribute_id: u16) -> Self {
        Self {
            direction,
            attribute_id,
        }
    }

    /// Return the reporting direction.
    #[must_use]
    pub const fn direction(&self) -> u8 {
        self.direction
    }

    /// Return the attribute identifier.
    #[must_use]
    pub const fn attribute_id(&self) -> u16 {
        self.attribute_id
    }
}
