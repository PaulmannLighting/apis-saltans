pub use date_code::DateCode;
pub use device_enabled::DeviceEnabled;
pub use physical_environment::PhysicalEnvironment;
pub use power_source::PowerSource;

mod date_code;
mod device_enabled;
mod physical_environment;
mod power_source;

/// Basic Cluster Attributes.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u16)]
pub enum Attribute {
    /// The ZCL version.
    ZclVersion(u8) = 0x0000,
    /// The application version.
    ApplicationVersion(u8) = 0x0001,
    /// The stack version.
    StackVersion(u8) = 0x0002,
    /// The hardware version.
    HwVersion(u8) = 0x0003,
    /// The manufacturer name.
    ManufacturerName(heapless::String<32>) = 0x0004,
    /// The model identifier.
    ModelIdentifier(heapless::String<32>) = 0x0005,
    /// The date code.
    DateCode(DateCode) = 0x0006,
    /// The power source.
    PowerSource(PowerSource) = 0x0007,
    /// The generic device class.
    LocationDescription(heapless::String<16>) = 0x0010,
    /// The physical environment.
    PhysicalEnvironment(PhysicalEnvironment) = 0x0011,
    /// The device enabled state.
    DeviceEnabled(DeviceEnabled) = 0x0012,
    /// The alarm mask.
    AlarmMask(u8) = 0x0013,
    /// The disable local configuration attribute.
    DisableLocalConfig(u8) = 0x0014,
    /// The cluster revision.
    SwBuildId(heapless::String<16>) = 0x4000,
}
