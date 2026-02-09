use le_stream::{FromLeStream, ToLeStream};
use zigbee::{Cluster, Direction};

use super::CLUSTER_ID;
use crate::Command;

/// Response to a [`GetAlarm`](super::GetAlarm) command.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, FromLeStream, ToLeStream)]
pub struct GetAlarmResponse {
    status: u8,
    alarm_code: u8,
    cluster_id: u16,
    timestamp: u32,
}

impl GetAlarmResponse {
    /// Creates a new `GetAlarmResponse` with the given status, alarm code, cluster ID, and timestamp.
    #[must_use]
    pub const fn new(status: u8, alarm_code: u8, cluster_id: u16, timestamp: u32) -> Self {
        Self {
            status,
            alarm_code,
            cluster_id,
            timestamp,
        }
    }

    /// Returns the status of the `GetAlarm` command.
    #[must_use]
    pub const fn status(self) -> u8 {
        self.status
    }

    /// Returns the alarm code of the earliest generated entry.
    #[must_use]
    pub const fn alarm_code(self) -> u8 {
        self.alarm_code
    }

    /// Returns the cluster ID associated with the earliest generated entry.
    #[must_use]
    pub const fn cluster_id(self) -> u16 {
        self.cluster_id
    }

    /// Returns the timestamp of when the earliest generated entry was created.
    #[must_use]
    pub const fn timestamp(self) -> u32 {
        // TODO: Is this really a `Uint32` or actually a `UtcTime`?
        self.timestamp
    }
}

impl Cluster for GetAlarmResponse {
    const ID: u16 = CLUSTER_ID;
}

impl Command for GetAlarmResponse {
    const ID: u8 = 0x01;
    const DIRECTION: Direction = Direction::ServerToClient;
}
