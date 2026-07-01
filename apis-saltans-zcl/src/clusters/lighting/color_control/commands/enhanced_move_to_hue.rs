use apis_saltans_core::types::Uint16;
use apis_saltans_core::{ClusterId, ClusterSpecific};
use le_stream::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::clusters::lighting::color_control::move_to_hue::Direction;
use crate::{Command, Options};

/// Command to move a light to a specific extended hue with a direction and transition time.
#[derive(Clone, Debug, Eq, Hash, PartialEq, FromLeStream, ToLeStream)]
pub struct EnhancedMoveToHue {
    enhanced_hue: u16,
    direction: u8,
    transition_time: Uint16,
    options: Options,
}

impl EnhancedMoveToHue {
    /// Create a new `EnhancedMoveToHue` command.
    #[must_use]
    pub const fn new(
        enhanced_hue: u16,
        direction: Direction,
        transition_time: Uint16,
        options: Options,
    ) -> Self {
        Self {
            enhanced_hue,
            direction: direction as u8,
            transition_time,
            options,
        }
    }

    /// Return the enhanced hue value.
    #[must_use]
    pub const fn enhanced_hue(&self) -> u16 {
        self.enhanced_hue
    }

    /// Return the direction of the hue change.
    ///
    /// # Errors
    ///
    /// Returns an error if the direction value is not a valid `Direction`.
    pub fn direction(&self) -> Result<Direction, u8> {
        Direction::from_u8(self.direction).ok_or(self.direction)
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

impl ClusterSpecific for EnhancedMoveToHue {
    const CLUSTER: ClusterId = ClusterId::ColorControl;
}

impl Command for EnhancedMoveToHue {
    const ID: u8 = 0x40;
    const DIRECTION: apis_saltans_core::Direction = apis_saltans_core::Direction::ClientToServer;
}

impl From<EnhancedMoveToHue> for crate::Cluster {
    fn from(command: EnhancedMoveToHue) -> Self {
        Self::ColorControl(command.into())
    }
}
