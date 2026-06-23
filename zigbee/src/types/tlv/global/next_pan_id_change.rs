use le_stream::{FromLeStream, ToLeStream};

use crate::types::tlv::Tag;

/// Next PAN ID TLV structure.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
pub struct NextPanIdChange {
    pan_id: u16,
}

impl NextPanIdChange {
    /// Create a new `NextPanIdChange`.
    #[must_use]
    pub const fn new(pan_id: u16) -> Self {
        Self { pan_id }
    }

    /// Get the nex PAN ID.
    #[must_use]
    pub const fn pan_id(self) -> u16 {
        self.pan_id
    }
}

impl Tag for NextPanIdChange {
    const TAG: u8 = 67;
}

impl From<NextPanIdChange> for u16 {
    fn from(value: NextPanIdChange) -> Self {
        value.pan_id
    }
}

impl From<u16> for NextPanIdChange {
    fn from(pan_id: u16) -> Self {
        Self { pan_id }
    }
}
