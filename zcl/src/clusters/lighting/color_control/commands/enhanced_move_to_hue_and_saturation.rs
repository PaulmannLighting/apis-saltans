use core::time::Duration;

use zigbee::{Cluster, Direction, FromDeciSeconds};

use crate::Command;
use crate::clusters::lighting::color_control::CLUSTER_ID;

/// Command to move a light to a specific hue and saturation with enhanced precision.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct EnhancedMoveToHueAndSaturation {
    enhanced_hue: u16,
    saturation: u8,
    transition_time: u16,
}

impl EnhancedMoveToHueAndSaturation {
    /// Create a new `EnhancedMoveToHueAndSaturation` command.
    #[must_use]
    pub const fn new(enhanced_hue: u16, saturation: u8, transition_time: u16) -> Self {
        Self {
            enhanced_hue,
            saturation,
            transition_time,
        }
    }

    /// Return the enhanced hue value.
    #[must_use]
    pub const fn enhanced_hue(self) -> u16 {
        self.enhanced_hue
    }

    /// Return the saturation value.
    #[must_use]
    pub const fn saturation(self) -> u8 {
        self.saturation
    }

    /// Return the transition time.
    #[must_use]
    pub fn transition_time(self) -> Duration {
        Duration::from_deci_seconds(self.transition_time)
    }
}

impl Cluster for EnhancedMoveToHueAndSaturation {
    const ID: u16 = CLUSTER_ID;
}

impl Command for EnhancedMoveToHueAndSaturation {
    const ID: u8 = 0x43;
    const DIRECTION: Direction = Direction::ClientToServer;
}
