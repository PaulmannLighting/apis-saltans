pub use mains_information::MainsInformation;

mod mains_information;

const ATTRIBUTE_ID_MASK: u16 = 0xFFF0;
const ATTRIBUTE_VALUE_MASK: u16 = 0x000F;

#[repr(u16)]
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
