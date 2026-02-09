//! Commands of the Alarms cluster.

use zigbee::Cluster;
use zigbee_macros::ParseZclFrame;

pub use self::get_alarm::GetAlarm;
pub use self::reset_alarm::ResetAlarm;
pub use self::reset_alarm_log::ResetAlarmLog;
pub use self::reset_all_alarms::ResetAllAlarms;
use super::CLUSTER_ID;
use crate::CommandId;

mod get_alarm;
mod reset_alarm;
mod reset_alarm_log;
mod reset_all_alarms;

/// Commands of the Alarms cluster.
#[derive(Clone, Debug, Eq, PartialEq, Hash, ParseZclFrame)]
pub enum Command {
    /// Get Alarm command.
    GetAlarm(GetAlarm),
    /// Reset Alarm command.
    ResetAlarm(ResetAlarm),
    /// Reset All Alarms command.
    ResetAllAlarms(ResetAllAlarms),
    /// Reset Alarm Log command.
    ResetAlarmLog(ResetAlarmLog),
}

impl Cluster for Command {
    const ID: u16 = CLUSTER_ID;
}

impl CommandId for Command {
    fn command_id(&self) -> u8 {
        match self {
            Self::GetAlarm(cmd) => cmd.command_id(),
            Self::ResetAlarm(cmd) => cmd.command_id(),
            Self::ResetAllAlarms(cmd) => cmd.command_id(),
            Self::ResetAlarmLog(cmd) => cmd.command_id(),
        }
    }
}
