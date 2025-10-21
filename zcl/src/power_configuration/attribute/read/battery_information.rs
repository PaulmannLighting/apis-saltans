use le_stream::{FromLeStream, FromLeStreamTagged};
use repr_discriminant::ReprDiscriminant;
use zb::types::Uint8;

const MASK: u16 = 0x000f;

/// Information about the battery status of a device.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u16)]
#[derive(ReprDiscriminant)]
pub enum BatteryInformation {
    /// Battery voltage in 100mV.
    BatteryVoltage(Uint8) = 0x0000,
    /// Battery percentage remaining.
    BatteryPercentageRemaining(Uint8) = 0x0001,
}

impl FromLeStreamTagged for BatteryInformation {
    type Tag = u16;

    fn from_le_stream_tagged<T>(tag: Self::Tag, bytes: T) -> Result<Option<Self>, Self::Tag>
    where
        T: Iterator<Item = u8>,
    {
        match tag & MASK {
            0x0000 => Ok(Uint8::from_le_stream(bytes).map(Self::BatteryVoltage)),
            0x0001 => Ok(Uint8::from_le_stream(bytes).map(Self::BatteryPercentageRemaining)),
            _ => Err(tag),
        }
    }
}
