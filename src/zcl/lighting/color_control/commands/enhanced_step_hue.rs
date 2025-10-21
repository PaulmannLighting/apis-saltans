use core::time::Duration;

use crate::zcl::constants::DECI_SECONDS_PER_MILLISECOND;
use crate::zcl::lighting::color_control::step_hue::Mode;
use crate::zcl::lighting::color_control::CLUSTER_ID;
use crate::{Cluster, Command};

/// Command to step a light's hue in an enhanced way, allowing for more control over the size.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct EnhancedStepHue {
    mode: Mode,
    size: u16,
    transition_time: u16,
}

impl EnhancedStepHue {
    /// Create a new `EnhancedStepHue` command.
    #[must_use]
    pub const fn new(mode: Mode, size: u16, transition_time: u16) -> Self {
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
    pub const fn size(self) -> u16 {
        self.size
    }

    /// Return the transition time in deci-seconds.
    #[must_use]
    pub fn transition_time(self) -> Duration {
        Duration::from_millis(u64::from(self.transition_time) * DECI_SECONDS_PER_MILLISECOND)
    }
}

impl Cluster for EnhancedStepHue {
    const ID: u16 = CLUSTER_ID;
}

impl Command for EnhancedStepHue {
    const ID: u8 = 0x42;
}
