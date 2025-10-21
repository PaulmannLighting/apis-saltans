use crate::lighting::color_control::CLUSTER_ID;
use crate::{Cluster, Command};

/// Command to move a light's color.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct MoveColor {
    rate_x: i16,
    rate_y: i16,
}

impl MoveColor {
    /// Create a new `MoveColor` command.
    #[must_use]
    pub const fn new(rate_x: i16, rate_y: i16) -> Self {
        Self { rate_x, rate_y }
    }

    /// Return the rate of change in the X color component.
    #[must_use]
    pub const fn rate_x(self) -> i16 {
        self.rate_x
    }

    /// Return the rate of change in the Y color component.
    #[must_use]
    pub const fn rate_y(self) -> i16 {
        self.rate_y
    }
}

impl Cluster for MoveColor {
    const ID: u16 = CLUSTER_ID;
}

impl Command for MoveColor {
    const ID: u8 = 0x08;
}
