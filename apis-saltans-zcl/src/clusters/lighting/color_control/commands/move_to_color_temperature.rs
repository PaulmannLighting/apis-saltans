use le_stream::{FromLeStream, ToLeStream};
use apis_saltans_core::types::Uint16;
use apis_saltans_core::{ClusterId, Cluster, Direction};

use crate::{Command, Options};

/// Command to move a light's color temperature to a specific value in mireds.
#[derive(Clone, Debug, Eq, Hash, PartialEq, FromLeStream, ToLeStream)]
pub struct MoveToColorTemperature {
    mireds: u16,
    transition_time: Uint16,
    options: Options,
}

impl MoveToColorTemperature {
    /// Create a new `MoveToColorTemperature` command.
    #[must_use]
    pub const fn new(mireds: u16, transition_time: Uint16, options: Options) -> Self {
        Self {
            mireds,
            transition_time,
            options,
        }
    }

    /// Return the color temperature in mireds.
    #[must_use]
    pub const fn mireds(&self) -> u16 {
        self.mireds
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

impl Cluster<ClusterId> for MoveToColorTemperature {
    const ID: ClusterId = ClusterId::ColorControl;
}

impl Command for MoveToColorTemperature {
    const ID: u8 = 0x0a;
    const DIRECTION: Direction = Direction::ClientToServer;
}

impl From<MoveToColorTemperature> for crate::Cluster {
    fn from(command: MoveToColorTemperature) -> Self {
        Self::ColorControl(command.into())
    }
}
