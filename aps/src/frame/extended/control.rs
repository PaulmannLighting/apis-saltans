use bitflags::bitflags;
use le_stream::{FromLeStream, ToLeStream};

/// Control field of the extended header.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
#[repr(transparent)]
pub struct Control(u8);

bitflags! {
    impl Control: u8 {
        /// Indicates whether the extended header is present.
        const FRAGMENTATION = 0b1100_0000;

        /// Reserved.
        const RESERVED = 0b0011_1111;

        /// Frame is the first frame of a fragmented transmission.
        const FIRST_FRAGMENT = 0b0100_0000;

        /// Frame is a follow-up frame of a fragmented transmission.
        const FOLLOWUP_FRAGMENT = 0b1000_0000;
    }
}
