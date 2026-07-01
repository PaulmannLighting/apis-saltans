use le_stream::{FromLeStream, ToLeStream};
use apis_saltans_core::types::Uint16;
use apis_saltans_core::{ClusterId, ClusterSpecific, Direction};

use crate::{Command, Options};

/// Command to move a light to a specific hue and saturation.
#[derive(Clone, Debug, Eq, Hash, PartialEq, FromLeStream, ToLeStream)]
pub struct MoveToHueAndSaturation {
    hue: u8,
    saturation: u8,
    transition_time: Uint16,
    options: Options,
}

impl MoveToHueAndSaturation {
    /// Create a new `MoveToHueAndSaturation` command.
    #[must_use]
    pub const fn new(hue: u8, saturation: u8, transition_time: Uint16, options: Options) -> Self {
        Self {
            hue,
            saturation,
            transition_time,
            options,
        }
    }

    /// Return the hue value.
    #[must_use]
    pub const fn hue(&self) -> u8 {
        self.hue
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

impl ClusterSpecific for MoveToHueAndSaturation {
    const CLUSTER: ClusterId = ClusterId::ColorControl;
}

impl Command for MoveToHueAndSaturation {
    const ID: u8 = 0x06;
    const DIRECTION: Direction = Direction::ClientToServer;
}

impl From<MoveToHueAndSaturation> for crate::Cluster {
    fn from(command: MoveToHueAndSaturation) -> Self {
        Self::ColorControl(command.into())
    }
}
