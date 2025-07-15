use crate::zcl::lighting::color_control::CLUSTER_ID;
use crate::zcl::lighting::color_control::move_hue::Mode;
use crate::zcl::{Cluster, Command};

/// Command to move a light's hue.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct MoveHue {
    mode: Mode,
    /// Steps per second.
    rate: u8,
}

impl MoveHue {
    /// Create a new `MoveHue` command.
    #[must_use]
    pub const fn new(mode: Mode, rate: u8) -> Self {
        Self { mode, rate }
    }

    /// Return the misc.
    #[must_use]
    pub const fn mode(self) -> Mode {
        self.mode
    }

    /// Return the rate of hue change in steps per second.
    #[must_use]
    pub const fn rate(self) -> u8 {
        self.rate
    }
}

impl Cluster for MoveHue {
    const ID: u16 = CLUSTER_ID;
}

impl Command for MoveHue {
    const ID: u8 = 0x01;
}
