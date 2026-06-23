use le_stream::FromLeStream;
use repr_discriminant::ReprDiscriminant;
use zigbee::types::Uint8;

const MASK: u16 = 0x000f;

/// Information about the battery status of a device.
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    expect(clippy::unsafe_derive_deserialize)
)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u16)]
#[derive(ReprDiscriminant)]
pub enum Information {
    /// Battery percentage remaining.
    PercentageRemaining(Uint8) = 0x0001,
}

impl Information {
    pub(crate) fn from_le_stream_tagged<T>(tag: u16, bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        match tag & MASK {
            0x0001 => Uint8::from_le_stream(bytes).map(Self::PercentageRemaining),
            _ => None,
        }
    }
}
