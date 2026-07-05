//! Attributes of the Power Configuration cluster.

use apis_saltans_core::ClusterId;
use apis_saltans_core::types::{String as ZclString, Uint8, Uint16};

pub use self::types::{BatteryAlarmMask, BatteryAlarmState, BatterySize, MainsAlarmMask};
use crate::macros::zcl_attributes;

mod types;

zcl_attributes! {
    cluster: ClusterId::PowerConfiguration;

    /// The actual RMS or DC voltage currently applied to the device, in units of 100 mV.
    MainsVoltage = 0x0000: Uint16 { R },
    /// The measured mains frequency.
    MainsFrequency = 0x0001: Uint8 { R },

    /// Mains alarms that may be generated.
    MainsAlarmMask = 0x0010: MainsAlarmMask { R, W },
    /// Lower mains voltage alarm threshold, in units of 100 mV.
    MainsVoltageMinThreshold = 0x0011: Uint16 { R, W },
    /// Upper mains voltage alarm threshold, in units of 100 mV.
    MainsVoltageMaxThreshold = 0x0012: Uint16 { R, W },
    /// Time in seconds that mains voltage may remain beyond its thresholds before an alarm is generated.
    MainsVoltageDwellTripPoint = 0x0013: Uint16 { R, W },

    /// Current measured battery voltage, in units of 100 mV.
    BatteryVoltage = 0x0020: Uint8 { R },
    /// Remaining battery life as a half-integer percentage.
    BatteryPercentageRemaining = 0x0021: Uint8 { R, P },

    /// Battery manufacturer name.
    BatteryManufacturer = 0x0030: ZclString<16> { R, W },
    /// Battery size.
    BatterySize = 0x0031: BatterySize { R, W },
    /// Battery ampere-hour rating, in units of 10 mAh.
    BatteryAHrRating = 0x0032: Uint16 { R, W },
    /// Number of battery cells used to power the device.
    BatteryQuantity = 0x0033: Uint8 { R, W },
    /// Rated battery voltage, in units of 100 mV.
    BatteryRatedVoltage = 0x0034: Uint8 { R, W },
    /// Battery alarms that must be generated.
    BatteryAlarmMask = 0x0035: BatteryAlarmMask { R, W },
    /// Low battery voltage alarm threshold, in units of 100 mV.
    BatteryVoltageMinThreshold = 0x0036: Uint8 { R, W },
    /// First low battery voltage alarm threshold, in units of 100 mV.
    BatteryVoltageThreshold1 = 0x0037: Uint8 { R, W },
    /// Second low battery voltage alarm threshold, in units of 100 mV.
    BatteryVoltageThreshold2 = 0x0038: Uint8 { R, W },
    /// Third low battery voltage alarm threshold, in units of 100 mV.
    BatteryVoltageThreshold3 = 0x0039: Uint8 { R, W },
    /// Low battery percentage alarm threshold.
    BatteryPercentageMinThreshold = 0x003a: Uint8 { R, W },
    /// First low battery percentage alarm threshold.
    BatteryPercentageThreshold1 = 0x003b: Uint8 { R, W },
    /// Second low battery percentage alarm threshold.
    BatteryPercentageThreshold2 = 0x003c: Uint8 { R, W },
    /// Third low battery percentage alarm threshold.
    BatteryPercentageThreshold3 = 0x003d: Uint8 { R, W },
    /// Current battery alarm state.
    BatteryAlarmState = 0x003e: BatteryAlarmState { R, P },

    /// Current measured battery source 2 voltage, in units of 100 mV.
    Battery2Voltage = 0x0040: Uint8 { R },
    /// Remaining battery source 2 life as a half-integer percentage.
    Battery2PercentageRemaining = 0x0041: Uint8 { R, P },

    /// Battery source 2 manufacturer name.
    Battery2Manufacturer = 0x0050: ZclString<16> { R, W },
    /// Battery source 2 size.
    Battery2Size = 0x0051: BatterySize { R, W },
    /// Battery source 2 ampere-hour rating, in units of 10 mAh.
    Battery2AHrRating = 0x0052: Uint16 { R, W },
    /// Number of battery source 2 cells used to power the device.
    Battery2Quantity = 0x0053: Uint8 { R, W },
    /// Rated battery source 2 voltage, in units of 100 mV.
    Battery2RatedVoltage = 0x0054: Uint8 { R, W },
    /// Battery source 2 alarms that must be generated.
    Battery2AlarmMask = 0x0055: BatteryAlarmMask { R, W },
    /// Low battery source 2 voltage alarm threshold, in units of 100 mV.
    Battery2VoltageMinThreshold = 0x0056: Uint8 { R, W },
    /// First low battery source 2 voltage alarm threshold, in units of 100 mV.
    Battery2VoltageThreshold1 = 0x0057: Uint8 { R, W },
    /// Second low battery source 2 voltage alarm threshold, in units of 100 mV.
    Battery2VoltageThreshold2 = 0x0058: Uint8 { R, W },
    /// Third low battery source 2 voltage alarm threshold, in units of 100 mV.
    Battery2VoltageThreshold3 = 0x0059: Uint8 { R, W },
    /// Low battery source 2 percentage alarm threshold.
    Battery2PercentageMinThreshold = 0x005a: Uint8 { R, W },
    /// First low battery source 2 percentage alarm threshold.
    Battery2PercentageThreshold1 = 0x005b: Uint8 { R, W },
    /// Second low battery source 2 percentage alarm threshold.
    Battery2PercentageThreshold2 = 0x005c: Uint8 { R, W },
    /// Third low battery source 2 percentage alarm threshold.
    Battery2PercentageThreshold3 = 0x005d: Uint8 { R, W },
    /// Current battery source 2 alarm state.
    Battery2AlarmState = 0x005e: BatteryAlarmState { R, P },

    /// Current measured battery source 3 voltage, in units of 100 mV.
    Battery3Voltage = 0x0060: Uint8 { R },
    /// Remaining battery source 3 life as a half-integer percentage.
    Battery3PercentageRemaining = 0x0061: Uint8 { R, P },

    /// Battery source 3 manufacturer name.
    Battery3Manufacturer = 0x0070: ZclString<16> { R, W },
    /// Battery source 3 size.
    Battery3Size = 0x0071: BatterySize { R, W },
    /// Battery source 3 ampere-hour rating, in units of 10 mAh.
    Battery3AHrRating = 0x0072: Uint16 { R, W },
    /// Number of battery source 3 cells used to power the device.
    Battery3Quantity = 0x0073: Uint8 { R, W },
    /// Rated battery source 3 voltage, in units of 100 mV.
    Battery3RatedVoltage = 0x0074: Uint8 { R, W },
    /// Battery source 3 alarms that must be generated.
    Battery3AlarmMask = 0x0075: BatteryAlarmMask { R, W },
    /// Low battery source 3 voltage alarm threshold, in units of 100 mV.
    Battery3VoltageMinThreshold = 0x0076: Uint8 { R, W },
    /// First low battery source 3 voltage alarm threshold, in units of 100 mV.
    Battery3VoltageThreshold1 = 0x0077: Uint8 { R, W },
    /// Second low battery source 3 voltage alarm threshold, in units of 100 mV.
    Battery3VoltageThreshold2 = 0x0078: Uint8 { R, W },
    /// Third low battery source 3 voltage alarm threshold, in units of 100 mV.
    Battery3VoltageThreshold3 = 0x0079: Uint8 { R, W },
    /// Low battery source 3 percentage alarm threshold.
    Battery3PercentageMinThreshold = 0x007a: Uint8 { R, W },
    /// First low battery source 3 percentage alarm threshold.
    Battery3PercentageThreshold1 = 0x007b: Uint8 { R, W },
    /// Second low battery source 3 percentage alarm threshold.
    Battery3PercentageThreshold2 = 0x007c: Uint8 { R, W },
    /// Third low battery source 3 percentage alarm threshold.
    Battery3PercentageThreshold3 = 0x007d: Uint8 { R, W },
    /// Current battery source 3 alarm state.
    Battery3AlarmState = 0x007e: BatteryAlarmState { R, P },
}
