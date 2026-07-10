use le_stream::{FromLeStream, ToLeStream};
use zb_core::types::Type;

use super::super::Selector;

/// Write attribute record of a Write Attributes Structured command.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
pub struct Record {
    attribute_id: u16,
    selector: Selector,
    data: Type,
}

impl Record {
    /// Create a structured write attribute record.
    #[must_use]
    pub const fn new(attribute_id: u16, selector: Selector, data: Type) -> Self {
        Self {
            attribute_id,
            selector,
            data,
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

    /// Return the typed attribute data.
    #[must_use]
    pub const fn data(&self) -> &Type {
        &self.data
    }
}
