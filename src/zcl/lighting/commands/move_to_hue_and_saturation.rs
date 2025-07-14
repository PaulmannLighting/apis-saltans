use crate::zcl::{Cluster, Command};
use std::time::Duration;

/// Amount of deci-seconds per millisecond.
const DECI_SECONDS_PER_MILLISECOND: u64 = 100;

/// Command to move a light to a specific hue and saturation.
pub struct MoveToHueAndSaturation {
    hue: u8,
    saturation: u8,
    /// The transition time in deci-seconds.
    transition_time: u16,
}

impl MoveToHueAndSaturation {
    /// Creates a new `MoveToHueAndSaturation` command.
    #[must_use]
    pub const fn new(hue: u8, saturation: u8, transition_time: u16) -> Self {
        Self {
            hue,
            saturation,
            transition_time,
        }
    }

    /// Returns the hue value.
    #[must_use]
    pub const fn hue(self) -> u8 {
        self.hue
    }

    /// Returns the saturation value.
    #[must_use]
    pub const fn saturation(self) -> u8 {
        self.saturation
    }

    /// Returns the transition time.
    #[must_use]
    pub fn transition_time(self) -> Duration {
        Duration::from_millis(u64::from(self.transition_time) * DECI_SECONDS_PER_MILLISECOND)
    }
}

impl Cluster for MoveToHueAndSaturation {
    const ID: u16 = 0x0300;
}

impl Command for MoveToHueAndSaturation {
    const ID: u8 = 0x06;
}
