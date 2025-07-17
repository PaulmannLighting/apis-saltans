use le_stream::FromLeStream;
pub use mains_information::MainsInformation;
use repr_discriminant::repr_discriminant;

mod mains_information;

const ATTRIBUTE_ID_MASK: u16 = 0xFFF0;
const ATTRIBUTE_VALUE_MASK: u16 = 0x000F;

#[repr_discriminant(u16)]
pub enum Attribute {
    /// Mains information.
    MainsInformation(MainsInformation) = 0x0000,
    /// Mains settings.
    MainsSettings = 0x0010,
    /// Battery information.
    BatteryInformation = 0x0020,
    /// Battery settings.
    BatterySettings = 0x0030,
    /// Battery source 2 information.
    BatterySource2Information = 0x0040,
    /// Battery source 2 settings.
    BatterySource2Settings = 0x0050,
    /// Battery source 3 information.
    BatterySource3Information = 0x0060,
    /// Battery source 3 settings.
    BatterySource3Settings = 0x0070,
}

impl FromLeStream for Attribute {
    fn from_le_stream<T>(mut bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        let id = u16::from_le_stream(&mut bytes)?;
        match id & ATTRIBUTE_ID_MASK {
            0x0000 => MainsInformation::from_le_stream(id & ATTRIBUTE_VALUE_MASK, &mut bytes)
                .map(Self::MainsInformation),
            0x0010 => Some(Self::MainsSettings),
            0x0020 => Some(Self::BatteryInformation),
            0x0030 => Some(Self::BatterySettings),
            0x0040 => Some(Self::BatterySource2Information),
            0x0050 => Some(Self::BatterySource2Settings),
            0x0060 => Some(Self::BatterySource3Information),
            0x0070 => Some(Self::BatterySource3Settings),
            _ => None,
        }
    }
}
