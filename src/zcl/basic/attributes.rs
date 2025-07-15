/// Basic Cluster Attributes.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Attribute {
    ZCLVersion(u8),
    ApplicationVersion(u8),
    StackVersion(u8),
    HWVersion(u8),
    ManufacturerName(String),
    ModelIdentifier(String),
    DateCode(String),
    PowerSource(u8),
    LocationDescription(String),
    PhysicalEnvironment(u8),
    DeviceEnabled(bool),
    AlarmMask(u8),
    DisableLocalConfig(u8),
    SwBuildId(String),
}
