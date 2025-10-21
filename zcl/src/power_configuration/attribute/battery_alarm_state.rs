use bitflags::bitflags;
use le_stream::derive::{FromLeStream, ToLeStream};

/// Battery alarm state flags.
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent)
)]
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
#[repr(transparent)]
pub struct BatteryAlarmState(u32);

bitflags! {
    impl BatteryAlarmState: u32 {
        /// `BatteryVoltageMinThreshold` or `BatteryPercentageMinThreshold` reached for Battery Source 1.
        const BATT_1_MIN_THRESHOLD = 0b1 << 0;
        /// `BatteryVoltageThreshold1` or `BatteryPercentageThreshold1` reached for Battery Source 1.
        const BATT_1_THRESHOLD_1 = 0b1 << 1;
        /// `BatteryVoltageThreshold2` or `BatteryPercentageThreshold2` reached for Battery Source 1.
        const BATT_1_THRESHOLD_2 = 0b1 << 2;
        /// `BatteryVoltageThreshold3` or `BatteryPercentageThreshold3` reached for Battery Source 1.
        const BATT_1_THRESHOLD_3 = 0b1 << 3;
        /// `BatteryVoltageMinThreshold` or `BatteryPercentageMinThreshold` reached for Battery Source 2.
        const BATT_2_MIN_THRESHOLD = 0b1 << 10;
        /// `BatteryVoltageThreshold1` or `BatteryPercentageThreshold1` reached for Battery Source 2.
        const BATT_2_THRESHOLD_1 = 0b1 << 11;
        /// `BatteryVoltageThreshold2` or `BatteryPercentageThreshold2` reached Battery Source 2.
        const BATT_2_THRESHOLD_2 = 0b1 << 12;
        /// `BatteryVoltageThreshold3` or `BatteryPercentageThreshold3` reached Battery Source 2.
        const BATT_2_THRESHOLD_3 = 0b1 << 13;
        /// `BatteryVoltageMinThreshold` or `BatteryPercentageMinThreshold` reached for Battery Source 3.
        const BATT_3_MIN_THRESHOLD = 0b1 << 20;
        /// `BatteryVoltageThreshold1` or `BatteryPercentageThreshold1` reached for Battery Source 3.
        const BATT_3_THRESHOLD_1 = 0b1 << 21;
        /// `BatteryVoltageThreshold2` or `BatteryPercentageThreshold2` reached Battery Source 3.
        const BATT_3_THRESHOLD_2 = 0b1 << 22;
        /// `BatteryVoltageThreshold3` or `BatteryPercentageThreshold3` reached Battery Source 3.
        const BATT_3_THRESHOLD_3 = 0b1 << 23;
        /// Mains power supply lost/unavailable (i.e., device is running on battery).
        const MAINS_POWER_SUPPLY_LOST = 0b1 << 30;
    }
}
