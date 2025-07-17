//! Power configuration cluster.

pub use attribute::{
    Attribute, BatteryAlarmMask, BatteryAlarmState, BatteryInformation, BatterySettings,
    BatterySize, MainsAlarmMask,
};
pub use battery_alarm::BatteryAlarm;

mod attribute;
mod battery_alarm;

const CLUSTER_ID: u16 = 0x0001;
