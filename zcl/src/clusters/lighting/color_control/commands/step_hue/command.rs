use core::time::Duration;

use zigbee::constants::DECI_SECONDS_PER_MILLISECOND;
use zigbee::{Cluster, Direction};

use crate::Command;
use crate::clusters::lighting::color_control::CLUSTER_ID;
use crate::clusters::lighting::color_control::step_hue::Mode;

/// Command to step a light's hue.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct StepHue {
    mode: Mode,
    size: u8,
    transition_time: u8,
}

impl StepHue {
    /// Create a new `StepHue` command.
    #[must_use]
    pub const fn new(mode: Mode, size: u8, transition_time: u8) -> Self {
        Self {
            mode,
            size,
            transition_time,
        }
    }

    /// Return the misc of hue step.
    #[must_use]
    pub const fn mode(self) -> Mode {
        self.mode
    }

    /// Return the size of hue step.
    #[must_use]
    pub const fn size(self) -> u8 {
        self.size
    }

    /// Return the transition time in deci-seconds.
    #[must_use]
    pub fn transition_time(self) -> Duration {
        Duration::from_millis(u64::from(self.transition_time) * DECI_SECONDS_PER_MILLISECOND)
    }
}

impl Cluster for StepHue {
    const ID: u16 = CLUSTER_ID;
}

impl Command for StepHue {
    const ID: u8 = 0x02;
    const DIRECTION: Direction = Direction::ClientToServer;
}
