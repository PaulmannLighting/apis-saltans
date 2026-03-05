//! Alarms cluster implementation.

pub use self::attribute::readable;
pub use self::commands::{Command, GetAlarm, ResetAlarm, ResetAlarmLog, ResetAllAlarms};
pub use self::table::Entry;

mod attribute;
mod commands;
mod table;

const CLUSTER_ID: u16 = 0x0009;
