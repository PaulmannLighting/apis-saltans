use le_stream::{FromLeStream, ToLeStream};
use zigbee::{Cluster, Direction};

use super::CLUSTER_ID;
use crate::Command;

/// Reset a specific alarm.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, FromLeStream, ToLeStream)]
pub struct ResetAlarm {
    code: u8,
    cluster_id: u16,
}

impl ResetAlarm {
    /// Creates a new `ResetAlarm` command with the given code and cluster ID.
    #[must_use]
    pub const fn new(code: u8, cluster_id: u16) -> Self {
        Self { code, cluster_id }
    }

    /// Returns the alarm code to reset.
    #[must_use]
    pub const fn code(self) -> u8 {
        self.code
    }

    /// Returns the cluster ID associated with the alarm to reset.
    #[must_use]
    pub const fn cluster_id(self) -> u16 {
        self.cluster_id
    }
}

impl Cluster for ResetAlarm {
    const ID: u16 = CLUSTER_ID;
}

impl Command for ResetAlarm {
    const ID: u8 = 0x00;
    const DIRECTION: Direction = Direction::ClientToServer;
}
