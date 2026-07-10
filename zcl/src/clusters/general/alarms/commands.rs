//! Commands of the Alarms cluster.

use apis_saltans_core::Cluster;

pub use self::alarm::Alarm;
pub use self::get_alarm::GetAlarm;
pub use self::get_alarm_response::GetAlarmResponse;
pub use self::reset_alarm::ResetAlarm;
pub use self::reset_alarm_log::ResetAlarmLog;
pub use self::reset_all_alarms::ResetAllAlarms;
use crate::macros::zcl_command_enum;

mod alarm;
mod get_alarm;
mod get_alarm_response;
mod reset_alarm;
mod reset_alarm_log;
mod reset_all_alarms;

// Commands of the Alarms cluster.
zcl_command_enum! {
    { Cluster::Alarms } => Alarms;
    GetAlarm(GetAlarm),
    ResetAlarm(ResetAlarm),
    ResetAllAlarms(ResetAllAlarms),
    ResetAlarmLog(ResetAlarmLog),
    Alarm(Alarm),
    GetAlarmResponse(GetAlarmResponse),
}
