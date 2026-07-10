//! Attributes of the Device Temperature Configuration cluster.

use zb_core::Cluster;
use zb_core::types::{Int16, Uint16, Uint24};

pub use self::types::AlarmMask;
use crate::macros::zcl_attributes;

mod types;

zcl_attributes! {
    cluster: Cluster::DeviceTemperatureConfiguration;

    /// Current temperature in degrees Celsius.
    CurrentTemperature = 0x0000: Int16 { R },
    /// Minimum temperature experienced in degrees Celsius.
    MinTempExperienced = 0x0001: Int16 { R },
    /// Maximum temperature experienced in degrees Celsius.
    MaxTempExperienced = 0x0002: Int16 { R },
    /// Total time the temperature was above the maximum threshold in hours.
    OverTempTotalDwell = 0x0003: Uint16 { R },
    /// Alarms mask for device temperature.
    DeviceTempAlarmMask = 0x0010: AlarmMask { R, W },
    /// Low temperature threshold in degrees Celsius.
    LowTempThreshold = 0x0011: Int16 { R, W },
    /// High temperature threshold in degrees Celsius.
    HighTempThreshold = 0x0012: Int16 { R, W },
    /// Low temperature dwell trip point in seconds.
    LowTempDwellTripPoint = 0x0013: Uint24 { R, W },
    /// High temperature dwell trip point in seconds.
    HighTempDwellTripPoint = 0x0014: Uint24 { R, W },
}
