use std::time::Duration;

use crate::zcl::Command;
use crate::zcl::constants::DECI_SECONDS_PER_MILLISECOND;
use crate::zcl::lighting::ColorControl;

/// Command to move a light to a specific hue and saturation.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct MoveToHueAndSaturation {
    hue: u8,
    saturation: u8,
    /// The transition time in deci-seconds.
    transition_time: u16,
}

impl MoveToHueAndSaturation {
    /// Create a new `MoveToHueAndSaturation` command.
    #[must_use]
    pub const fn new(hue: u8, saturation: u8, transition_time: u16) -> Self {
        Self {
            hue,
            saturation,
            transition_time,
        }
    }

    /// Return the hue value.
    #[must_use]
    pub const fn hue(self) -> u8 {
        self.hue
    }

    /// Return the saturation value.
    #[must_use]
    pub const fn saturation(self) -> u8 {
        self.saturation
    }

    /// Return the transition time.
    #[must_use]
    pub fn transition_time(self) -> Duration {
        Duration::from_millis(u64::from(self.transition_time) * DECI_SECONDS_PER_MILLISECOND)
    }
}

impl ColorControl for MoveToHueAndSaturation {}

impl Command for MoveToHueAndSaturation {
    const ID: u8 = 0x06;
}
