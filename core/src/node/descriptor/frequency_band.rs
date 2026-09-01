use bitflags::bitflags;
use le_stream::{FromLeStream, ToLeStream};

/// Supported frequency bands.
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent)
)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
pub struct FrequencyBand(u8);

bitflags! {
    impl FrequencyBand: u8 {
        /// From 868 MHz to 868.6 MHz.
        const FROM_868_TO_868_6_MHZ = 0b0000_0001;

        /// Deprecated incorrectly named alias of [`Self::FROM_868_TO_868_6_MHZ`].
        #[deprecated(note = "use FROM_868_TO_868_6_MHZ")]
        const FROM_863_TO_868_MHZ = Self::FROM_868_TO_868_6_MHZ.bits();

        /// From 902 MHz to 928 MHz.
        const FROM_902_TO_928_MHZ = 0b0000_0100;

        /// From 2400 MHz to 2483.5 MHz.
        const FROM_2400_TO_2483_5_MHZ = 0b0000_1000;

        /// GB Smart Energy Sub-GHz Band.
        const GB_SMART_ENERGY_SUB_GHZ = 0b0001_0000;

        /// Deprecated misspelling of [`Self::GB_SMART_ENERGY_SUB_GHZ`].
        #[deprecated(note = "use GB_SMART_ENERGY_SUB_GHZ")]
        const GB_SMART_ENEGERGY_SUB_GHZ = Self::GB_SMART_ENERGY_SUB_GHZ.bits();
    }
}

impl_bitflags_display_and_from_str!(FrequencyBand);
