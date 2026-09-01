use bitflags::bitflags;
use le_stream::{FromLeStream, ToLeStream};

/// Leave Request Flags.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, ToLeStream)]
#[repr(transparent)]
pub struct LeaveReqFlags(u8);

bitflags! {
    impl LeaveReqFlags: u8 {
        /// Rejoin flag.
        const REJOIN = 0b1000_0000;
        /// Remove children flag.
        const REMOVE_CHILDREN = 0b0100_0000;
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

#[cfg(test)]
mod tests {
    use le_stream::FromLeStream;

    use super::LeaveReqFlags;

    #[test]
    fn bit_assignments_match_wire_format() {
        assert_eq!(LeaveReqFlags::REMOVE_CHILDREN.bits(), 0b0100_0000);
        assert_eq!(LeaveReqFlags::REJOIN.bits(), 0b1000_0000);
    }

    #[test]
    fn parses_wire_flags() {
        let flags = LeaveReqFlags::from_le_stream([0b1100_0000].into_iter());

        assert_eq!(
            flags,
            Some(LeaveReqFlags::REMOVE_CHILDREN | LeaveReqFlags::REJOIN)
        );
    }
}
