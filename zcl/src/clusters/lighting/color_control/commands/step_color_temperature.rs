use core::time::Duration;

use le_stream::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;
use zigbee::{Cluster, Direction, FromDeciSeconds};

use crate::clusters::lighting::color_control::CLUSTER_ID;
use crate::clusters::lighting::color_control::step_hue::Mode;
use crate::{Command, Options};

/// Command to step a light's color temperature in a specified range.
#[derive(Clone, Debug, Eq, Hash, PartialEq, FromLeStream, ToLeStream)]
pub struct StepColorTemperature {
    mode: u8,
    size: u16,
    transition_time: u16,
    color_temp_min_mireds: u16,
    color_temp_max_mireds: u16,
    options: Options,
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
        options: Options,
    ) -> Self {
        Self {
            mode: mode as u8,
            size,
            transition_time,
            color_temp_min_mireds,
            color_temp_max_mireds,
            options,
        }
    }

    /// Return the mode of color temperature step.
    ///
    /// # Errors
    ///
    /// Returns the raw mode value if it cannot be converted into a `Mode` enum.
    pub fn mode(&self) -> Result<Mode, u8> {
        Mode::from_u8(self.mode).ok_or(self.mode)
    }

    /// Return the size of color temperature step.
    #[must_use]
    pub const fn size(&self) -> u16 {
        self.size
    }

    /// Return the transition time.
    #[must_use]
    pub fn transition_time(&self) -> Duration {
        Duration::from_deci_seconds(self.transition_time)
    }

    /// Return the minimum color temperature in mireds.
    #[must_use]
    pub const fn color_temp_min_mireds(&self) -> u16 {
        self.color_temp_min_mireds
    }

    /// Return the maximum color temperature in mireds.
    #[must_use]
    pub const fn color_temp_max_mireds(&self) -> u16 {
        self.color_temp_max_mireds
    }

    /// Return the options for the command.
    #[must_use]
    pub const fn options(&self) -> Options {
        self.options
    }
}

impl Cluster for StepColorTemperature {
    const ID: u16 = CLUSTER_ID;
}

impl Command for StepColorTemperature {
    const ID: u8 = 0x4c;
    const DIRECTION: Direction = Direction::ClientToServer;
}
