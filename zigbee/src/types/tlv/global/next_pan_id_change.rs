use std::iter::Chain;

use le_stream::{FromLeStream, ToLeStream};

use crate::types::tlv::Tag;

/// Next PAN ID TLV structure.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, FromLeStream)]
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

    fn size(&self) -> usize {
        2
    }
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

impl ToLeStream for NextPanIdChange {
    type Iter =
        Chain<Chain<<u8 as ToLeStream>::Iter, <u8 as ToLeStream>::Iter>, <u16 as ToLeStream>::Iter>;

    fn to_le_stream(self) -> Self::Iter {
        Self::TAG
            .to_le_stream()
            .chain(self.serialized_size().to_le_stream())
            .chain(self.pan_id.to_le_stream())
    }
}
