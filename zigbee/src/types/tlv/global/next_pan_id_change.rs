use le_stream::{FromLeStream, ToLeStream};

use crate::types::tlv::Tag;

/// Next PAN ID TLV structure.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
pub struct NextPanIdChange {
    next_channel: u16,
}

impl NextPanIdChange {
    /// Get the Next Channel.
    #[must_use]
    pub const fn next_channel(self) -> u16 {
        self.next_channel
    }
}

impl Tag for NextPanIdChange {
    const TAG: u8 = 67;
}
