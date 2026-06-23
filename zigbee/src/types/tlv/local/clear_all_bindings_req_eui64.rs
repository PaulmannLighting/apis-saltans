use le_stream::{FromLeStream, ToLeStream};
use macaddr::MacAddr8;

use crate::ByteSizedVec;
use crate::types::tlv::Tag;

/// Clear All Bindings Request EUI64 List.
#[derive(Clone, Debug, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
pub struct ClearAllBindingsReqEui64 {
    eui64s: ByteSizedVec<MacAddr8>,
}

impl ClearAllBindingsReqEui64 {
    /// Creates a new `ClearAllBindingsReqEui64`.
    #[must_use]
    pub const fn new(eui64s: ByteSizedVec<MacAddr8>) -> Self {
        Self { eui64s }
    }

    /// Returns a reference to the EUI64 list.
    #[must_use]
    pub fn eui64s(&self) -> &[MacAddr8] {
        &self.eui64s
    }
}

impl Tag for ClearAllBindingsReqEui64 {
    const TAG: u8 = 0x00;
}
