use crate::macros::zcl_attribute_newtype;

zcl_attribute_newtype! {
    /// Available battery alarms.
    pub bitflags BatteryAlarmMask(u8) => Map8 {
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
