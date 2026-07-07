use le_stream::{FromLeStream, ToLeStream};

use crate::types::tlv::Tag;
use crate::{ByteSizedVec, Eui64};

/// Clear All Bindings Request EUI64 List.
#[derive(Clone, Debug, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
pub struct ClearAllBindingsReqEui64 {
    eui64s: ByteSizedVec<Eui64>,
}

impl ClearAllBindingsReqEui64 {
    /// Creates a new `ClearAllBindingsReqEui64`.
    #[must_use]
    pub const fn new(eui64s: ByteSizedVec<Eui64>) -> Self {
        Self { eui64s }
    }

    /// Returns a reference to the EUI64 list.
    #[must_use]
    pub fn eui64s(&self) -> &[Eui64] {
        &self.eui64s
    }
}

impl Tag for ClearAllBindingsReqEui64 {
    const TAG: u8 = 0x00;
}
