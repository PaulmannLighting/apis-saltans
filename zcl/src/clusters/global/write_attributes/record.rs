use le_stream::{FromLeStream, ToLeStream};
use zb_core::types::Type;

/// Write Attributes record.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
pub struct Record {
    id: u16,
    typ: Type,
}

impl Record {
    /// Create a new record.
    #[must_use]
    pub const fn new(id: u16, typ: Type) -> Self {
        Self { id, typ }
    }
}
