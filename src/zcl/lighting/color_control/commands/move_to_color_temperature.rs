use core::time::Duration;

use crate::zcl::constants::DECI_SECONDS_PER_MILLISECOND;
use crate::zcl::lighting::color_control::CLUSTER_ID;
use crate::{Cluster, Command};

/// Command to move a light's color temperature to a specific value in mireds.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct MoveToColorTemperature {
    mireds: u16,
    transition_time: u16,
}

impl MoveToColorTemperature {
    /// Create a new `MoveToColorTemperature` command.
    #[must_use]
    pub const fn new(mireds: u16, transition_time: u16) -> Self {
        Self {
            mireds,
            transition_time,
        }
    }

    /// Return the color temperature in mireds.
    #[must_use]
    pub const fn mireds(self) -> u16 {
        self.mireds
    }

    /// Return the transition time.
    #[must_use]
    pub fn transition_time(self) -> Duration {
        Duration::from_millis(u64::from(self.transition_time) * DECI_SECONDS_PER_MILLISECOND)
    }
}

impl Cluster for MoveToColorTemperature {
    const ID: u16 = CLUSTER_ID;
}

impl Command for MoveToColorTemperature {
    const ID: u8 = 0x0a;
}
