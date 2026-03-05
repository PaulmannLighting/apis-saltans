use le_stream::{FromLeStream, FromLeStreamTagged};
use repr_discriminant::ReprDiscriminant;
use zigbee::types::Uint8;

const MASK: u16 = 0x000f;

/// Information about the battery status of a device.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u16)]
#[derive(ReprDiscriminant)]
pub enum Information {
    /// Battery percentage remaining.
    PercentageRemaining(Uint8) = 0x0001,
}

impl FromLeStreamTagged for Information {
    type Tag = u16;

    fn from_le_stream_tagged<T>(tag: Self::Tag, bytes: T) -> Result<Option<Self>, Self::Tag>
    where
        T: Iterator<Item = u8>,
    {
        match tag & MASK {
            0x0001 => Ok(Uint8::from_le_stream(bytes).map(Self::PercentageRemaining)),
            _ => Err(tag),
        }
    }
}
