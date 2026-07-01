use le_stream::{FromLeStream, ToLeStream};
use apis_saltans_core::types::Uint16;
use apis_saltans_core::{ClusterId, ClusterSpecific, Direction};

use crate::{Command, Options};

/// Command to move a light to a specific saturation.
#[derive(Clone, Debug, Eq, Hash, PartialEq, FromLeStream, ToLeStream)]
pub struct MoveToSaturation {
    saturation: u8,
    transition_time: Uint16,
    options: Options,
}

impl MoveToSaturation {
    /// Create a new `MoveToSaturation` command.
    #[must_use]
    pub const fn new(saturation: u8, transition_time: Uint16, options: Options) -> Self {
        Self {
            saturation,
            transition_time,
            options,
        }
    }

    /// Return the saturation value.
    #[must_use]
    pub const fn saturation(&self) -> u8 {
        self.saturation
    }

    /// Return the transition time, if any, in deciseconds.
    #[must_use]
    pub fn transition_time(&self) -> Option<u16> {
        self.transition_time.into()
    }

    /// Return the options for this command.
    #[must_use]
    pub const fn options(&self) -> Options {
        self.options
    }
}

impl ClusterSpecific for MoveToSaturation {
    const CLUSTER: ClusterId = ClusterId::ColorControl;
}

impl Command for MoveToSaturation {
    const ID: u8 = 0x03;
    const DIRECTION: Direction = Direction::ClientToServer;
}

impl From<MoveToSaturation> for crate::Cluster {
    fn from(command: MoveToSaturation) -> Self {
        Self::ColorControl(command.into())
    }
}
