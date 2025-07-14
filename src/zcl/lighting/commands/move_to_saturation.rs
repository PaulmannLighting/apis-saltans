use std::time::Duration;

use crate::zcl::{Cluster, Command, constants::DECI_SECONDS_PER_MILLISECOND};

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
        Duration::from_millis(u64::from(self.transition_time) * DECI_SECONDS_PER_MILLISECOND)
    }
}

impl Cluster for MoveToSaturation {
    const ID: u16 = 0x0300;
}

impl Command for MoveToSaturation {
    const ID: u8 = 0x03;
}
