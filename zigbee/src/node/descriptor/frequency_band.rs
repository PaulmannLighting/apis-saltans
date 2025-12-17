use bitflags::bitflags;
use le_stream::{FromLeStream, ToLeStream};

/// Supported frequency bands.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
pub struct FrequencyBand(u8);

bitflags! {
    impl FrequencyBand: u8 {
        const FROM_863_TO_868_MHZ = 0b0001_0000;
        const FROM_902_TO_928_MHZ = 0b0000_0100;
        const FROM_2400_TO_2483_5_MHZ = 0b0000_0010;
    }
}
