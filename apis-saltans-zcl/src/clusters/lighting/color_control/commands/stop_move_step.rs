use le_stream::{FromLeStream, ToLeStream};
use apis_saltans_core::{ClusterId, ClusterSpecific, Direction};

use crate::{Command, Options};

/// Command to stop a move step in a lighting device.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, FromLeStream, ToLeStream)]
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

impl ClusterSpecific for StopMoveStep {
    const CLUSTER: ClusterId = ClusterId::ColorControl;
}

impl Command for StopMoveStep {
    const ID: u8 = 47;
    const DIRECTION: Direction = Direction::ClientToServer;
}

impl From<StopMoveStep> for crate::Cluster {
    fn from(command: StopMoveStep) -> Self {
        Self::ColorControl(command.into())
    }
}
