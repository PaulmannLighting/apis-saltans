//! Commands of the Alarms cluster.

use le_stream::ToLeStream;
use zigbee::{ClusterId, ClusterSpecific, Direction};
use zigbee_macros::ParseZclFrame;

pub use self::alarm::Alarm;
pub use self::get_alarm::GetAlarm;
pub use self::get_alarm_response::GetAlarmResponse;
pub use self::reset_alarm::ResetAlarm;
pub use self::reset_alarm_log::ResetAlarmLog;
pub use self::reset_all_alarms::ResetAllAlarms;
use crate::{CommandDispatch, Scope};

mod alarm;
mod get_alarm;
mod get_alarm_response;
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
    /// Alarm command.
    Alarm(Alarm),
    /// Get Alarm Response command.
    GetAlarmResponse(GetAlarmResponse),
}

impl ClusterSpecific for Command {
    const CLUSTER: ClusterId = ClusterId::Alarms;
}

impl CommandDispatch for Command {
    fn command_id(&self) -> u8 {
        match self {
            Self::GetAlarm(cmd) => cmd.command_id(),
            Self::ResetAlarm(cmd) => cmd.command_id(),
            Self::ResetAllAlarms(cmd) => cmd.command_id(),
            Self::ResetAlarmLog(cmd) => cmd.command_id(),
            Self::Alarm(cmd) => cmd.command_id(),
            Self::GetAlarmResponse(cmd) => cmd.command_id(),
        }
    }

    fn scope(&self) -> Scope {
        match self {
            Self::GetAlarm(cmd) => cmd.scope(),
            Self::ResetAlarm(cmd) => cmd.scope(),
            Self::ResetAllAlarms(cmd) => cmd.scope(),
            Self::ResetAlarmLog(cmd) => cmd.scope(),
            Self::Alarm(cmd) => cmd.scope(),
            Self::GetAlarmResponse(cmd) => cmd.scope(),
        }
    }

    fn direction(&self) -> Direction {
        match self {
            Self::GetAlarm(cmd) => cmd.direction(),
            Self::ResetAlarm(cmd) => cmd.direction(),
            Self::ResetAllAlarms(cmd) => cmd.direction(),
            Self::ResetAlarmLog(cmd) => cmd.direction(),
            Self::Alarm(cmd) => cmd.direction(),
            Self::GetAlarmResponse(cmd) => cmd.direction(),
        }
    }

    fn disable_default_response(&self) -> bool {
        match self {
            Self::GetAlarm(cmd) => cmd.disable_default_response(),
            Self::ResetAlarm(cmd) => cmd.disable_default_response(),
            Self::ResetAllAlarms(cmd) => cmd.disable_default_response(),
            Self::ResetAlarmLog(cmd) => cmd.disable_default_response(),
            Self::Alarm(cmd) => cmd.disable_default_response(),
            Self::GetAlarmResponse(cmd) => cmd.disable_default_response(),
        }
    }
}

impl ToLeStream for Command {
    type Iter = Iter;

    fn to_le_stream(self) -> Self::Iter {
        match self {
            Self::GetAlarm(cmd) => Iter::GetAlarm(cmd.to_le_stream()),
            Self::ResetAlarm(cmd) => Iter::ResetAlarm(cmd.to_le_stream()),
            Self::ResetAllAlarms(cmd) => Iter::ResetAllAlarms(cmd.to_le_stream()),
            Self::ResetAlarmLog(cmd) => Iter::ResetAlarmLog(cmd.to_le_stream()),
            Self::Alarm(cmd) => Iter::Alarm(cmd.to_le_stream()),
            Self::GetAlarmResponse(cmd) => Iter::GetAlarmResponse(cmd.to_le_stream()),
        }
    }
}

#[derive(Debug)]
pub enum Iter {
    GetAlarm(<GetAlarm as ToLeStream>::Iter),
    ResetAlarm(<ResetAlarm as ToLeStream>::Iter),
    ResetAllAlarms(<ResetAllAlarms as ToLeStream>::Iter),
    ResetAlarmLog(<ResetAlarmLog as ToLeStream>::Iter),
    Alarm(<Alarm as ToLeStream>::Iter),
    GetAlarmResponse(<GetAlarmResponse as ToLeStream>::Iter),
}

impl Iterator for Iter {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::GetAlarm(iter) => iter.next(),
            Self::ResetAlarm(iter) => iter.next(),
            Self::ResetAllAlarms(iter) => iter.next(),
            Self::ResetAlarmLog(iter) => iter.next(),
            Self::Alarm(iter) => iter.next(),
            Self::GetAlarmResponse(iter) => iter.next(),
        }
    }
}
