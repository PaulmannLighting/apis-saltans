use bitflags::bitflags;
use le_stream::{FromLeStream, ToLeStream};

use super::frequency_band::FrequencyBand;
use super::logical_type::LogicalType;

/// First two bytes of the node descriptor.
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent)
)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
pub struct Flags(u16);

bitflags! {
    impl Flags: u16 {
        /// Logical type of the device.
        const LOGICAL_TYPE = 0b0000_0000_0000_0111;

        /// Deprecated complex descriptor availability field.
        const COMPLEX_DESCRIPTOR_AVAILABLE = 0b0000_0000_0000_1000;

        /// Deprecated user descriptor availability field.
        const USER_DESCRIPTOR_AVAILABLE = 0b0000_0000_0001_0000;

        /// APS-layer fragmentation support for Revision 23 and later devices.
        const FRAGMENTATION_SUPPORTED = 0b0000_0000_0010_0000;

        /// APS flags.
        const APS_FLAGS = 0b0000_0111_0000_0000;

        /// Frequency band.
        const FREQUENCY_BAND = 0b1111_1000_0000_0000;
    }
}

impl_bitflags_display_and_from_str!(Flags);

impl Flags {
    /// Returns the logical type.
    ///
    /// # Errors
    ///
    /// Returns an error if the logical type is set to the reserved bits.
    #[expect(
        clippy::cast_possible_truncation,
        reason = "the logical type field is three bits wide"
    )]
    pub fn logical_type(self) -> Result<LogicalType, u8> {
        let logical_type = ((self & Self::LOGICAL_TYPE).bits()
            >> Self::LOGICAL_TYPE.bits().trailing_zeros()) as u8;

        LogicalType::try_from(logical_type).map_err(|_| logical_type)
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
        ((self & Self::APS_FLAGS).bits() >> Self::APS_FLAGS.bits().trailing_zeros()) as u8
    }

    /// Returns the frequency band.
    #[must_use]
    pub fn frequency_band(self) -> FrequencyBand {
        #[expect(clippy::cast_possible_truncation)]
        FrequencyBand::from_bits_truncate(
            ((self & Self::FREQUENCY_BAND).bits() >> Self::FREQUENCY_BAND.bits().trailing_zeros())
                as u8,
        )
    }

    /// Sets the frequency band.
    pub fn set_frequency_band(&mut self, band: FrequencyBand) {
        *self = (*self & !Self::FREQUENCY_BAND)
            | Self(u16::from(band.bits()) << Self::FREQUENCY_BAND.bits().trailing_zeros());
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use alloc::string::ToString;

    use super::*;

    const DESCRIPTOR_FLAGS: &str = "COMPLEX_DESCRIPTOR_AVAILABLE | USER_DESCRIPTOR_AVAILABLE";

    #[test]
    fn bit_assignments_match_wire_format() {
        assert_eq!(Flags::LOGICAL_TYPE.bits(), 0x0007);
        assert_eq!(Flags::COMPLEX_DESCRIPTOR_AVAILABLE.bits(), 0x0008);
        assert_eq!(Flags::USER_DESCRIPTOR_AVAILABLE.bits(), 0x0010);
        assert_eq!(Flags::FRAGMENTATION_SUPPORTED.bits(), 0x0020);
        assert_eq!(Flags::APS_FLAGS.bits(), 0x0700);
        assert_eq!(Flags::FREQUENCY_BAND.bits(), 0xF800);
        assert_eq!(FrequencyBand::FROM_868_TO_868_6_MHZ.bits(), 0x01);
        assert_eq!(FrequencyBand::FROM_902_TO_928_MHZ.bits(), 0x04);
        assert_eq!(FrequencyBand::FROM_2400_TO_2483_5_MHZ.bits(), 0x08);
        assert_eq!(FrequencyBand::GB_SMART_ENERGY_SUB_GHZ.bits(), 0x10);
    }

    #[test]
    fn display_and_parsing_round_trip() {
        let flags = Flags::COMPLEX_DESCRIPTOR_AVAILABLE | Flags::USER_DESCRIPTOR_AVAILABLE;
        let displayed = flags.to_string();
        let parsed = displayed.parse::<Flags>();

        assert_eq!(displayed, DESCRIPTOR_FLAGS);
        assert!(matches!(parsed, Ok(parsed_flags) if parsed_flags == flags));
    }

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
            FrequencyBand::FROM_868_TO_868_6_MHZ | FrequencyBand::GB_SMART_ENERGY_SUB_GHZ,
        );
        assert!(flags.contains(Flags::COMPLEX_DESCRIPTOR_AVAILABLE));
        assert!(!flags.contains(Flags::USER_DESCRIPTOR_AVAILABLE));
        assert!(!flags.contains(Flags::APS_FLAGS));
        assert!(!flags.contains(Flags::FREQUENCY_BAND));
        assert_eq!(flags.logical_type(), Ok(LogicalType::Coordinator));
        assert_eq!(
            flags.frequency_band(),
            FrequencyBand::FROM_868_TO_868_6_MHZ | FrequencyBand::GB_SMART_ENERGY_SUB_GHZ
        );
    }
}
