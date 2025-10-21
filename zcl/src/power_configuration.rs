//! Power configuration cluster.

pub use attribute::{
    BatteryAlarmMask, BatteryAlarmState, BatterySize, MainsAlarmMask, read, write,
};
pub use battery_alarm::BatteryAlarm;

mod attribute;
mod battery_alarm;

const CLUSTER_ID: u16 = 0x0001;
