//! Power configuration cluster.

pub use self::attribute::{
    BatteryAlarmMask, BatteryAlarmState, BatterySize, MainsAlarmMask, readable, reportable,
    writable,
};
pub use self::battery_alarm::BatteryAlarm;

mod attribute;
pub mod attributes;
mod battery_alarm;
