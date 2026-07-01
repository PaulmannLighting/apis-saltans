use apis_saltans_core::types::Type;
use le_stream::{FromLeStream, ToLeStream};

/// Write Attributes record.
#[derive(Clone, Debug, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
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
