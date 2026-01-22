use le_stream::{FromLeStream, ToLeStream};
use zigbee::{Cluster, Direction};

use super::CLUSTER_ID;
use crate::Command;

/// Move to the closest frequency command.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
pub struct MoveToClosestFrequency {
    frequency: u16,
}

impl MoveToClosestFrequency {
    /// Creates a new `MoveToClosestFrequency` command.
    #[must_use]
    pub const fn new(frequency: u16) -> Self {
        Self { frequency }
    }

    /// Get the frequency.
    #[must_use]
    pub const fn frequency(self) -> u16 {
        self.frequency
    }
}

impl Cluster for MoveToClosestFrequency {
    const ID: u16 = CLUSTER_ID;
}

impl Command for MoveToClosestFrequency {
    const ID: u8 = 0x08;
    const DIRECTION: Direction = Direction::ClientToServer;
}
