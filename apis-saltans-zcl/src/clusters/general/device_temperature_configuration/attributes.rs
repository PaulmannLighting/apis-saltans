//! Attributes of the Device Temperature Configuration cluster.

use apis_saltans_core::ClusterId;
use apis_saltans_core::types::{Type, Uint16, Uint24};

use crate::macros::zcl_attributes;

zcl_attributes! {
    cluster: ClusterId::DeviceTemperatureConfiguration;

    /// Current temperature in degrees Celsius.
    CurrentTemperature = 0x0000: Type { R },
    /// Minimum temperature experienced in degrees Celsius.
    MinTempExperienced = 0x0001: Type { R },
    /// Maximum temperature experienced in degrees Celsius.
    MaxTempExperienced = 0x0002: Type { R },
    /// Total time the temperature was above the maximum threshold in hours.
    OverTempTotalDwell = 0x0003: Uint16 { R },
    /// Alarms mask for device temperature.
    DeviceTempAlarmMask = 0x0010: Type { R, W },
    /// Low temperature threshold in degrees Celsius.
    LowTempThreshold = 0x0011: Type { R, W },
    /// High temperature threshold in degrees Celsius.
    HighTempThreshold = 0x0012: Type { R, W },
    /// Low temperature dwell trip point in seconds.
    LowTempDwellTripPoint = 0x0013: Uint24 { R, W },
    /// High temperature dwell trip point in seconds.
    HighTempDwellTripPoint = 0x0014: Uint24 { R, W },
}
