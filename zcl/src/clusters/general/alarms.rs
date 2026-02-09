//! Alarms cluster implementation.

pub use self::commands::{Command, GetAlarm, ResetAlarm, ResetAlarmLog, ResetAllAlarms};
pub use self::table::Entry;

pub mod attributes;
mod commands;
mod table;

const CLUSTER_ID: u16 = 0x0009;
