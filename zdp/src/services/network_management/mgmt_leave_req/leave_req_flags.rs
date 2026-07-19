use bitflags::bitflags;
use le_stream::{FromLeStream, ToLeStream};

/// Leave Request Flags.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, ToLeStream)]
#[repr(transparent)]
pub struct LeaveReqFlags(u8);

bitflags! {
    impl LeaveReqFlags: u8 {
        /// Rejoin flag.
        const REJOIN = 0b0000_0001;
        /// Remove children flag.
        const REMOVE_CHILDREN = 0b0000_0010;
    }
}

impl core::fmt::Display for LeaveReqFlags {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        bitflags::parser::to_writer(self, formatter)
    }
}

impl core::str::FromStr for LeaveReqFlags {
    type Err = bitflags::parser::ParseError;

    fn from_str(flags: &str) -> Result<Self, Self::Err> {
        bitflags::parser::from_str(flags)
    }
}

impl FromLeStream for LeaveReqFlags {
    fn from_le_stream<T>(bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        u8::from_le_stream(bytes).map(Self::from_bits_truncate)
    }
}
