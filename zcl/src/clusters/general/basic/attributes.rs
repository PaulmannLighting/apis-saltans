//! Attributes of the Basic cluster.

use apis_saltans_core::Cluster;
use apis_saltans_core::types::{Bool, OctStr, String, Uint8};

#[allow(unused_imports)]
pub use self::date_code::{DateCode, ParseError};
pub use self::types::{
    AlarmMask, DisableLocalConfig, GenericDeviceClass, GenericDeviceType, PhysicalEnvironment,
    PowerSource,
};
use crate::macros::zcl_attributes;

mod date_code;
mod types;

zcl_attributes! {
    cluster: Cluster::Basic;

    /// The ZCL version.
    ZclVersion = 0x0000: Uint8 { R },
    /// The application version.
    ApplicationVersion = 0x0001: Uint8 { R },
    /// The stack version.
    StackVersion = 0x0002: Uint8 { R },
    /// The hardware version.
    HwVersion = 0x0003: Uint8 { R },
    /// The manufacturer's name.
    ManufacturerName = 0x0004: String<32> { R },
    /// The model identifier.
    ModelIdentifier = 0x0005: String<32> { R },
    /// The date code.
    DateCode = 0x0006: DateCode { R },
    /// The power source.
    PowerSource = 0x0007: PowerSource { R },
    /// The generic device class.
    GenericDeviceClass = 0x0008: GenericDeviceClass { R },
    /// The generic device type.
    GenericDeviceType = 0x0009: GenericDeviceType { R },
    /// The product code.
    ProductCode = 0x000a: OctStr { R },
    /// The product URL.
    ProductUrl = 0x000b: String { R },
    /// The manufacturer version details.
    ManufacturerVersionDetails = 0x000c: String { R },
    /// The serial number.
    SerialNumber = 0x000d: String { R },
    /// The product label.
    ProductLabel = 0x000e: String { R },
    /// The location description.
    LocationDescription = 0x0010: String<16> { R, W },
    /// The physical environment.
    PhysicalEnvironment = 0x0011: PhysicalEnvironment { R, W },
    /// The device enabled state.
    DeviceEnabled = 0x0012: Bool { R, W },
    /// The alarm mask.
    AlarmMask = 0x0013: AlarmMask { R, W },
    /// Flags to disable local configuration.
    DisableLocalConfig = 0x0014: DisableLocalConfig { R, W },
    /// The software build ID.
    SwBuildId = 0x4000: String<16> { R },
}
