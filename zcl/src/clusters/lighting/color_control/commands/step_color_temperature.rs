use core::time::Duration;

use zigbee::constants::DECI_SECONDS_PER_MILLISECOND;
use zigbee::{Cluster, Direction};

use crate::Command;
use crate::clusters::lighting::color_control::CLUSTER_ID;
use crate::clusters::lighting::color_control::step_hue::Mode;

/// Command to step a light's color temperature in a specified range.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct StepColorTemperature {
    mode: Mode,
    size: u16,
    transition_time: u16,
    color_temp_min_mireds: u16,
    color_temp_max_mireds: u16,
}

impl StepColorTemperature {
    /// Create a new `StepColorTemperature` command.
    #[must_use]
    pub const fn new(
        mode: Mode,
        size: u16,
        transition_time: u16,
        color_temp_min_mireds: u16,
        color_temp_max_mireds: u16,
    ) -> Self {
        Self {
            mode,
            size,
            transition_time,
            color_temp_min_mireds,
            color_temp_max_mireds,
        }
    }

    /// Return the mode of color temperature step.
    #[must_use]
    pub const fn mode(self) -> Mode {
        self.mode
    }

    /// Return the size of color temperature step.
    #[must_use]
    pub const fn size(self) -> u16 {
        self.size
    }

    /// Return the transition time.
    #[must_use]
    pub fn transition_time(self) -> Duration {
        Duration::from_millis(u64::from(self.transition_time) * DECI_SECONDS_PER_MILLISECOND)
    }

    /// Return the minimum color temperature in mireds.
    #[must_use]
    pub const fn color_temp_min_mireds(self) -> u16 {
        self.color_temp_min_mireds
    }

    /// Return the maximum color temperature in mireds.
    #[must_use]
    pub const fn color_temp_max_mireds(self) -> u16 {
        self.color_temp_max_mireds
    }
}

impl Cluster for StepColorTemperature {
    const ID: u16 = CLUSTER_ID;
}

impl Command for StepColorTemperature {
    const ID: u8 = 0x4c;
    const DIRECTION: Direction = Direction::ClientToServer;
}
