use le_stream::{FromLeStream, ToLeStream};

/// Available battery sizes.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
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

impl FromLeStream for BatterySize {
    fn from_le_stream<T>(mut bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        match u8::from_le_stream(&mut bytes)? {
            0x00 => Some(Self::NoBattery),
            0x01 => Some(Self::BuiltInBattery),
            0x02 => Some(Self::Other),
            0x03 => Some(Self::AA),
            0x04 => Some(Self::AAA),
            0x05 => Some(Self::C),
            0x06 => Some(Self::D),
            0x07 => Some(Self::CR2),
            0x08 => Some(Self::CR123A),
            0xff => Some(Self::Unknown),
            _ => None,
        }
    }
}

impl ToLeStream for BatterySize {
    type Iter = <u8 as ToLeStream>::Iter;

    fn to_le_stream(self) -> Self::Iter {
        (self as u8).to_le_stream()
    }
}
