use core::time::Duration;

use zigbee::{Cluster, Direction, FromDeciSeconds};

use crate::Command;
use crate::clusters::lighting::color_control::CLUSTER_ID;

/// Command to move a light to a specific saturation.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct MoveToSaturation {
    saturation: u8,
    transition_time: u16,
}

impl MoveToSaturation {
    /// Create a new `MoveToSaturation` command.
    #[must_use]
    pub const fn new(saturation: u8, transition_time: u16) -> Self {
        Self {
            saturation,
            transition_time,
        }
    }

    /// Return the saturation value.
    #[must_use]
    pub const fn saturation(self) -> u8 {
        self.saturation
    }

    /// Return the transition time.
    #[must_use]
    pub fn transition_time(self) -> Duration {
        Duration::from_deci_seconds(self.transition_time)
    }
}

impl Cluster for MoveToSaturation {
    const ID: u16 = CLUSTER_ID;
}

impl Command for MoveToSaturation {
    const ID: u8 = 0x03;
    const DIRECTION: Direction = Direction::ClientToServer;
}
