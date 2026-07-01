use le_stream::{FromLeStream, ToLeStream};
use apis_saltans_core::types::Uint16;
use apis_saltans_core::{ClusterId, Cluster, Direction};

use crate::{Command, Options};

/// Command to move a light to a specific hue and saturation with enhanced precision.
#[derive(Clone, Debug, Eq, Hash, PartialEq, FromLeStream, ToLeStream)]
pub struct EnhancedMoveToHueAndSaturation {
    enhanced_hue: u16,
    saturation: u8,
    transition_time: Uint16,
    options: Options,
}

impl EnhancedMoveToHueAndSaturation {
    /// Create a new `EnhancedMoveToHueAndSaturation` command.
    #[must_use]
    pub const fn new(
        enhanced_hue: u16,
        saturation: u8,
        transition_time: Uint16,
        options: Options,
    ) -> Self {
        Self {
            enhanced_hue,
            saturation,
            transition_time,
            options,
        }
    }

    /// Return the enhanced hue value.
    #[must_use]
    pub const fn enhanced_hue(&self) -> u16 {
        self.enhanced_hue
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

impl Cluster<ClusterId> for EnhancedMoveToHueAndSaturation {
    const ID: ClusterId = ClusterId::ColorControl;
}

impl Command for EnhancedMoveToHueAndSaturation {
    const ID: u8 = 0x43;
    const DIRECTION: Direction = Direction::ClientToServer;
}

impl From<EnhancedMoveToHueAndSaturation> for crate::Cluster {
    fn from(command: EnhancedMoveToHueAndSaturation) -> Self {
        Self::ColorControl(command.into())
    }
}
