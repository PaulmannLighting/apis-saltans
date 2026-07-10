use le_stream::{FromLeStream, ToLeStream};

use super::super::Selector;

/// Write status record of a Write Attributes Structured response.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
pub struct Status {
    code: u8,
    attribute_id: Option<u16>,
    selector: Option<Selector>,
}

impl Status {
    /// Create a structured write status record.
    #[must_use]
    pub const fn new(status: u8, attribute_id: Option<u16>, selector: Option<Selector>) -> Self {
        Self {
            code: status,
            attribute_id,
            selector,
        }
    }

    /// Return the raw status code.
    #[must_use]
    pub const fn status(&self) -> u8 {
        self.code
    }

    /// Return the attribute identifier, if present.
    #[must_use]
    pub const fn attribute_id(&self) -> Option<u16> {
        self.attribute_id
    }

    /// Return the selector, if present.
    #[must_use]
    pub const fn selector(&self) -> Option<&Selector> {
        self.selector.as_ref()
    }
}
