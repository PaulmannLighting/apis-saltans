use le_stream::{FromLeStream, ToLeStream};

use crate::types::tlv::Tag;

/// Next PAN ID TLV structure.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
pub struct NextPanIdChange {
    pan_id: u16,
}

impl NextPanIdChange {
    /// Get the nex PAN ID.
    #[must_use]
    pub const fn pan_id(self) -> u16 {
        self.pan_id
    }
}

impl Tag for NextPanIdChange {
    const TAG: u8 = 67;
}
