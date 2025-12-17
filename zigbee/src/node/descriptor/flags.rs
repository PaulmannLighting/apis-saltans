use bitflags::bitflags;
use le_stream::{FromLeStream, ToLeStream};

use super::frequency_band::FrequencyBand;
use super::logical_type::LogicalType;

/// First two bytes of the node descriptor.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
pub struct Flags(u16);

bitflags! {
    impl Flags: u16 {
        /// Logical type of the device.
        const LOGICAL_TYPE = 0b1110_0000_0000_0000;
        /// Complex descriptor availability.
        const COMPLEX_DESCRIPTOR_AVAILABLE = 0b0001_0000_0000_0000;
        /// User descriptor availability.
        const USER_DESCRIPTOR_AVAILABLE = 0b0000_1000_0000_0000;
        /// APS flags.
        const APS_FLAGS = 0b0000_0000_1110_0000;
        /// Frequency band.
        const FREQUENCY_BAND = 0b0000_0000_0001_1111;
    }
}

impl Flags {
    /// Returns the logical type.
    ///
    /// # Errors
    ///
    /// Returns an error if the logical type is set to the reserved bits.
    pub fn logical_type(self) -> Result<LogicalType, u8> {
        LogicalType::try_from(((self & Self::LOGICAL_TYPE).bits() >> 13) as u8)
    }

    /// Sets the logical type.
    pub fn set_logical_type(&mut self, logical_type: LogicalType) {
        *self = (*self & !Self::LOGICAL_TYPE)
            | Self(u16::from(logical_type as u8) << Self::LOGICAL_TYPE.bits().trailing_zeros());
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

    /// Returns the APS flags.
    #[must_use]
    #[expect(clippy::cast_possible_truncation)]
    pub fn aps_flags(self) -> u8 {
        ((self & Self::APS_FLAGS).bits() >> 5) as u8
    }

    /// Returns the frequency band.
    #[must_use]
    pub fn frequency_band(self) -> FrequencyBand {
        #[expect(clippy::cast_possible_truncation)]
        FrequencyBand::from_bits_truncate((self & Self::FREQUENCY_BAND).bits() as u8)
    }

    /// Sets the frequency band.
    pub fn set_frequency_band(&mut self, band: FrequencyBand) {
        *self = (*self & !Self::FREQUENCY_BAND) | Self(u16::from(band.bits()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flags_modification() {
        let mut flags = Flags::COMPLEX_DESCRIPTOR_AVAILABLE;

        flags.set_logical_type(LogicalType::Router);
        assert!(flags.contains(Flags::COMPLEX_DESCRIPTOR_AVAILABLE));
        assert!(!flags.contains(Flags::USER_DESCRIPTOR_AVAILABLE));
        assert!(!flags.contains(Flags::APS_FLAGS));
        assert!(!flags.contains(Flags::FREQUENCY_BAND));
        assert_eq!(flags.logical_type(), Ok(LogicalType::Router));

        flags.set_logical_type(LogicalType::Coordinator);
        assert!(flags.contains(Flags::COMPLEX_DESCRIPTOR_AVAILABLE));
        assert!(!flags.contains(Flags::USER_DESCRIPTOR_AVAILABLE));
        assert!(!flags.contains(Flags::APS_FLAGS));
        assert!(!flags.contains(Flags::FREQUENCY_BAND));
        assert_eq!(flags.logical_type(), Ok(LogicalType::Coordinator));

        flags.set_logical_type(LogicalType::EndDevice);
        assert!(flags.contains(Flags::COMPLEX_DESCRIPTOR_AVAILABLE));
        assert!(!flags.contains(Flags::USER_DESCRIPTOR_AVAILABLE));
        assert!(!flags.contains(Flags::APS_FLAGS));
        assert!(!flags.contains(Flags::FREQUENCY_BAND));
        assert_eq!(flags.logical_type(), Ok(LogicalType::EndDevice));

        flags.set_frequency_band(FrequencyBand::FROM_2400_TO_2483_5_MHZ);
        assert!(flags.contains(Flags::COMPLEX_DESCRIPTOR_AVAILABLE));
        assert!(!flags.contains(Flags::USER_DESCRIPTOR_AVAILABLE));
        assert!(!flags.contains(Flags::APS_FLAGS));
        assert!(!flags.contains(Flags::FREQUENCY_BAND));
        assert_eq!(
            flags.frequency_band(),
            FrequencyBand::FROM_2400_TO_2483_5_MHZ
        );

        flags.set_logical_type(LogicalType::Coordinator);
        assert!(flags.contains(Flags::COMPLEX_DESCRIPTOR_AVAILABLE));
        assert!(!flags.contains(Flags::USER_DESCRIPTOR_AVAILABLE));
        assert!(!flags.contains(Flags::APS_FLAGS));
        assert!(!flags.contains(Flags::FREQUENCY_BAND));
        assert_eq!(flags.logical_type(), Ok(LogicalType::Coordinator));
        assert_eq!(
            flags.frequency_band(),
            FrequencyBand::FROM_2400_TO_2483_5_MHZ
        );

        flags.set_frequency_band(
            FrequencyBand::FROM_863_TO_868_MHZ | FrequencyBand::GB_SMART_ENEGERGY_SUB_GHZ,
        );
        assert!(flags.contains(Flags::COMPLEX_DESCRIPTOR_AVAILABLE));
        assert!(!flags.contains(Flags::USER_DESCRIPTOR_AVAILABLE));
        assert!(!flags.contains(Flags::APS_FLAGS));
        assert!(!flags.contains(Flags::FREQUENCY_BAND));
        assert_eq!(flags.logical_type(), Ok(LogicalType::Coordinator));
        assert_eq!(
            flags.frequency_band(),
            FrequencyBand::FROM_863_TO_868_MHZ | FrequencyBand::GB_SMART_ENEGERGY_SUB_GHZ
        );
    }
}
