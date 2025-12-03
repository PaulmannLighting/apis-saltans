use bitflags::bitflags;
use le_stream::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use super::fragmentation::Fragmentation;

/// Control field of the extended header.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
#[repr(transparent)]
pub struct Control(u8);

bitflags! {
    impl Control: u8 {
        /// Indicates whether the extended header is present.
        const FRAGMENTATION = 0b1100_0000;
    }
}

impl Control {
    /// Returns the fragmentation field.
    #[must_use]
    pub fn fragmentation(self) -> Option<Fragmentation> {
        Fragmentation::from_u8((self.bits() & Self::FRAGMENTATION.bits()) >> 6)
    }
}
