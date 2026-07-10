use le_stream::{FromLeStream, ToLeStream};

use super::super::Selector;

/// Attribute record of a Read Attributes Structured command.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
pub struct Record {
    attribute_id: u16,
    selector: Selector,
}

impl Record {
    /// Create a structured read attribute record.
    #[must_use]
    pub const fn new(attribute_id: u16, selector: Selector) -> Self {
        Self {
            attribute_id,
            selector,
        }
    }

    /// Return the attribute identifier.
    #[must_use]
    pub const fn attribute_id(&self) -> u16 {
        self.attribute_id
    }

    /// Return the selector.
    #[must_use]
    pub const fn selector(&self) -> &Selector {
        &self.selector
    }
}
