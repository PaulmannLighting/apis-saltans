use le_stream::{FromLeStream, ToLeStream};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

/// Available battery sizes.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromPrimitive)]
#[repr(u8)]
pub enum BatterySize {
    /// No battery is present.
    NoBattery = 0x00,
    /// Built-in battery.
    BuiltInBattery = 0x01,
    /// Other battery types.
    Other = 0x02,
    /// AA battery.
    AA = 0x03,
    /// AAA battery.
    AAA = 0x04,
    /// C battery.
    C = 0x05,
    /// D battery.
    D = 0x06,
    /// CR2 battery.
    CR2 = 0x07,
    /// CR123A battery.
    CR123A = 0x08,
    /// 9V battery.
    Unknown = 0xff,
}

impl TryFrom<u8> for BatterySize {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or(value)
    }
}

impl FromLeStream for BatterySize {
    fn from_le_stream<T>(mut bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        u8::from_le_stream(&mut bytes).and_then(Self::from_u8)
    }
}

impl ToLeStream for BatterySize {
    type Iter = <u8 as ToLeStream>::Iter;

    fn to_le_stream(self) -> Self::Iter {
        (self as u8).to_le_stream()
    }
}
