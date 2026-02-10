use zigbee::{Cluster, Direction};

use crate::clusters::lighting::color_control::CLUSTER_ID;
use crate::clusters::lighting::color_control::move_hue::Mode;
use crate::{Command, Options};

/// Command to move a light's color temperature.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct MoveColorTemperature {
    mode: Mode,
    rate: u16,
    color_temp_min_mireds: u16,
    color_temp_max_mireds: u16,
    options: Options,
}

impl MoveColorTemperature {
    /// Create a new `MoveColorTemperature` command.
    #[must_use]
    pub const fn new(
        mode: Mode,
        rate: u16,
        color_temp_min_mireds: u16,
        color_temp_max_mireds: u16,
        options: Options,
    ) -> Self {
        Self {
            mode,
            rate,
            color_temp_min_mireds,
            color_temp_max_mireds,
            options,
        }
    }

    /// Return the mode of color temperature movement.
    #[must_use]
    pub const fn mode(&self) -> Mode {
        self.mode
    }

    /// Return the rate of color temperature change in mireds per second.
    #[must_use]
    pub const fn rate(&self) -> u16 {
        self.rate
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

impl Cluster for MoveColorTemperature {
    const ID: u16 = CLUSTER_ID;
}

impl Command for MoveColorTemperature {
    const ID: u8 = 0x4b;
    const DIRECTION: Direction = Direction::ClientToServer;
}
