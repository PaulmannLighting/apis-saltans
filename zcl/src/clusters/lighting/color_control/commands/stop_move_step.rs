use zigbee::{Cluster, Direction};

use crate::clusters::lighting::color_control::CLUSTER_ID;
use crate::{Command, Options};

/// Command to stop a move step in a lighting device.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct StopMoveStep {
    options: Options,
}

impl StopMoveStep {
    /// Create a new `StopMoveStep` command.
    #[must_use]
    pub const fn new(options: Options) -> Self {
        Self { options }
    }

    /// Return the options for the command.
    #[must_use]
    pub const fn options(self) -> Options {
        self.options
    }
}

impl Cluster for StopMoveStep {
    const ID: u16 = CLUSTER_ID;
}

impl Command for StopMoveStep {
    const ID: u8 = 47;
    const DIRECTION: Direction = Direction::ClientToServer;
}
