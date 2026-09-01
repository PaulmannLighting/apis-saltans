use bitflags::bitflags;
use le_stream::{FromLeStream, ToLeStream};

/// Control field of the extended header.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
#[repr(transparent)]
pub struct Control(u8);

bitflags! {
    impl Control: u8 {
        /// Fragmentation sub-field mask.
        const FRAGMENTATION = 0b0000_0011;

        /// Reserved.
        const RESERVED = 0b1111_1100;

        /// Frame is the first frame of a fragmented transmission.
        const FIRST_FRAGMENT = 0b0000_0001;

        /// Frame is a follow-up frame of a fragmented transmission.
        const FOLLOWUP_FRAGMENT = 0b0000_0010;
    }
}

impl core::fmt::Display for Control {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        bitflags::parser::to_writer(self, formatter)
    }
}

impl core::str::FromStr for Control {
    type Err = bitflags::parser::ParseError;

    fn from_str(flags: &str) -> Result<Self, Self::Err> {
        bitflags::parser::from_str(flags)
    }
}

#[cfg(test)]
mod tests {
    use super::Control;

    #[test]
    fn bit_assignments_match_wire_format() {
        assert_eq!(Control::FRAGMENTATION.bits(), 0b0000_0011);
        assert_eq!(Control::FIRST_FRAGMENT.bits(), 0b0000_0001);
        assert_eq!(Control::FOLLOWUP_FRAGMENT.bits(), 0b0000_0010);
        assert_eq!(Control::RESERVED.bits(), 0b1111_1100);
    }
}
