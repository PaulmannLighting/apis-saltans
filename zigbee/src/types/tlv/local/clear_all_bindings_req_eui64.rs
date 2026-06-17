use core::ops::Deref;

use le_stream::{FromLeStream, ToLeStream};
use macaddr::MacAddr8;

use crate::types::tlv::{Tag, TlvVec};

/// Clear All Bindings Request EUI64 List.
#[derive(Clone, Debug, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
pub struct ClearAllBindingsReqEui64 {
    eui64s: TlvVec<MacAddr8>,
}

impl ClearAllBindingsReqEui64 {
    /// Creates a new `ClearAllBindingsReqEui64`.
    pub fn new(eui64s: TlvVec<MacAddr8>) -> Self {
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

    fn size(&self) -> usize {
        self.eui64s.len()
    }
}

impl Deref for ClearAllBindingsReqEui64 {
    type Target = [MacAddr8];

    fn deref(&self) -> &Self::Target {
        &self.eui64s
    }
}
