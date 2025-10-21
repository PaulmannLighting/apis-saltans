use bitflags::bitflags;

use super::frequency_band::FrequencyBand;
use super::logical_type::LogicalType;

/// First two bytes of the node descriptor.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct Flags(u16);

bitflags! {
    impl Flags: u16 {
        const LOGICAL_TYPE = 0b1110_0000_0000_0000;
        const COMPLEX_DESCRIPTOR_AVAILABLE = 0b0001_0000_0000_0000;
        const USER_DESCRIPTOR_AVAILABLE = 0b0000_1000_0000_0000;
        const RESERVED = 0b0000_0111_0000_0000;
        const APS_FLAGS = 0b0000_0000_1110_0000;
        const FREQUENCY_BAND = 0b0000_0000_0001_1111;
    }
}

impl Flags {
    /// Returns the logical type.
    ///
    /// Returns an error if the logical type is set to the reserved bits.
    pub fn logical_type(self) -> Result<LogicalType, u8> {
        match ((self & Self::LOGICAL_TYPE).bits() >> 13) as u8 {
            0b000 => Ok(LogicalType::Coordinator),
            0b001 => Ok(LogicalType::Router),
            0b010 => Ok(LogicalType::EndDevice),
            reserved => Err(reserved),
        }
    }

    /// Returns whether the complex descriptor is available.
    #[must_use]
    pub const fn complex_descriptor_available(self) -> bool {
        self.contains(Self::COMPLEX_DESCRIPTOR_AVAILABLE)
    }

    /// Returns whether the user descriptor is available.
    #[must_use]
    pub const fn user_descriptor_available(self) -> bool {
        self.contains(Self::USER_DESCRIPTOR_AVAILABLE)
    }

    /// Returns the reserved bytes.
    #[must_use]
    pub fn reserved(self) -> u8 {
        ((self & Self::RESERVED).bits() >> 8) as u8
    }

    /// Returns the APS flags.
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn aps_flags(self) -> u8 {
        ((self & Self::APS_FLAGS).bits() >> 5) as u8
    }

    /// Returns the frequency band.
    #[must_use]
    pub fn frequency_band(self) -> FrequencyBand {
        #[allow(clippy::cast_possible_truncation)]
        FrequencyBand::from_bits_truncate((self & Self::FREQUENCY_BAND).bits() as u8)
    }
}
