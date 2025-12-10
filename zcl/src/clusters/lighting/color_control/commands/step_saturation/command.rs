use core::time::Duration;

use zigbee::constants::DECI_SECONDS_PER_MILLISECOND;
use zigbee::{Cluster, Command, Direction};

use crate::clusters::lighting::color_control::CLUSTER_ID;
use crate::clusters::lighting::color_control::step_saturation::Mode;

/// Command to step a light to a specific hue.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct StepSaturation {
    mode: Mode,
    size: u8,
    transition_time: u8,
}

impl StepSaturation {
    /// Create a new `StepSaturation` command.
    #[must_use]
    pub const fn new(mode: Mode, size: u8, transition_time: u8) -> Self {
        Self {
            mode,
            size,
            transition_time,
        }
    }

    /// Return the misc of saturation step.
    #[must_use]
    pub const fn mode(self) -> Mode {
        self.mode
    }

    /// Return the size of saturation step.
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

impl Cluster for StepSaturation {
    const ID: u16 = CLUSTER_ID;
}

impl Command for StepSaturation {
    const ID: u8 = 0x04;
    const DIRECTION: Direction = Direction::ClientToServer;
}
