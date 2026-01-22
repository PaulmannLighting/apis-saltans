use core::time::Duration;

use le_stream::{FromLeStream, ToLeStream};
use zigbee::types::Uint16;
use zigbee::{Cluster, Direction, FromDeciSeconds};

use super::CLUSTER_ID;
use crate::Command;

/// Move to level command.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
pub struct MoveToLevel {
    level: u8,
    transition_time: Uint16,
    options_mask: u8,
    options_override: u8,
}

impl MoveToLevel {
    /// Creates a new `MoveToLevel` command.
    #[must_use]
    pub const fn new(
        level: u8,
        transition_time: Uint16,
        options_mask: u8,
        options_override: u8,
    ) -> Self {
        Self {
            level,
            transition_time,
            options_mask,
            options_override,
        }
    }

    /// Get the level.
    #[must_use]
    pub const fn level(self) -> u8 {
        self.level
    }

    /// Get the transition time.
    #[must_use]
    pub fn transition_time(self) -> Option<Duration> {
        Option::<u16>::from(self.transition_time).map(Duration::from_deci_seconds)
    }

    /// Get the options mask.
    #[must_use]
    pub const fn options_mask(self) -> u8 {
        self.options_mask
    }

    /// Get the options override.
    #[must_use]
    pub const fn options_override(self) -> u8 {
        self.options_override
    }
}

impl Cluster for MoveToLevel {
    const ID: u16 = CLUSTER_ID;
}

impl Command for MoveToLevel {
    const ID: u8 = 0x00;
    const DIRECTION: Direction = Direction::ClientToServer;
}
