use bitflags::bitflags;
use le_stream::{FromLeStream, ToLeStream};

/// Available battery alarms.
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent)
)]
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
#[repr(transparent)]
pub struct BatteryAlarmMask(u8);

bitflags! {
    impl BatteryAlarmMask: u8 {
        /// `BatteryVoltageMinThreshold` value has been reached.
        const BATTERY_VOLTAGE_TOO_LOW = 0b0000_0001;
        /// `BatteryVoltageThreshold1` or `BatteryPercentageThreshold1` value has been reached.
        const BATTERY_ALARM_1 = 0b0000_0010;
        /// `BatteryVoltageThreshold2` or `BatteryPercentageThreshold2` value has been reached.
        const BATTERY_ALARM_2 = 0b0000_0100;
        /// `BatteryVoltageThreshold3` or `BatteryPercentageThreshold3` value has been reached.
        const BATTERY_ALARM_3 = 0b0000_1000;
    }
}
