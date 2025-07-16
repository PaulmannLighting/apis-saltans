pub use date_code::DateCode;
pub use power_source::PowerSource;

mod date_code;
mod physical_environment;
mod power_source;

/// Basic Cluster Attributes.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u16)]
pub enum Attribute {
    ZCLVersion(u8) = 0x0000,
    ApplicationVersion(u8) = 0x0001,
    StackVersion(u8) = 0x0002,
    HWVersion(u8) = 0x0003,
    ManufacturerName(heapless::String<32>) = 0x0004,
    ModelIdentifier(heapless::String<32>) = 0x0005,
    DateCode(DateCode) = 0x0006,
    PowerSource(PowerSource) = 0x0007,
    LocationDescription(heapless::String<16>) = 0x0010,
    PhysicalEnvironment(u8) = 0x0011,
    DeviceEnabled(bool) = 0x0012,
    AlarmMask(u8) = 0x0013,
    DisableLocalConfig(u8) = 0x0014,
    SwBuildId(heapless::String<16>) = 0x4000,
}
