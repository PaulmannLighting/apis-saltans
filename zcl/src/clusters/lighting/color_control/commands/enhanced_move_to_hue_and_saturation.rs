use core::num::TryFromIntError;
use core::time::Duration;

use le_stream::{FromLeStream, ToLeStream};
use zigbee::{Cluster, Direction, FromDeciSeconds, IntoDeciSeconds};

use crate::clusters::lighting::color_control::CLUSTER_ID;
use crate::{Command, Options};

/// Command to move a light to a specific hue and saturation with enhanced precision.
#[derive(Clone, Debug, Eq, Hash, PartialEq, FromLeStream, ToLeStream)]
pub struct EnhancedMoveToHueAndSaturation {
    enhanced_hue: u16,
    saturation: u8,
    transition_time: u16,
    options: Options,
}

impl EnhancedMoveToHueAndSaturation {
    /// Create a new `EnhancedMoveToHueAndSaturation` command.
    #[must_use]
    pub const fn new(
        enhanced_hue: u16,
        saturation: u8,
        transition_time: u16,
        options: Options,
    ) -> Self {
        Self {
            enhanced_hue,
            saturation,
            transition_time,
            options,
        }
    }

    /// Try to create a new `EnhancedMoveToHueAndSaturation` command.
    ///
    /// # Errors
    ///
    /// Returns an [`TryFromIntError`] if the resulting deci-seconds value cannot fit in a `u16`.
    pub fn try_new(
        enhanced_hue: u16,
        saturation: u8,
        transition_time: Duration,
        options: Options,
    ) -> Result<Self, TryFromIntError> {
        transition_time
            .into_deci_seconds()
            .try_into()
            .map(|transition_time| Self::new(enhanced_hue, saturation, transition_time, options))
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

    /// Return the transition time.
    #[must_use]
    pub fn transition_time(&self) -> Duration {
        Duration::from_deci_seconds(self.transition_time)
    }

    /// Return the options for this command.
    #[must_use]
    pub const fn options(&self) -> Options {
        self.options
    }
}

impl Cluster for EnhancedMoveToHueAndSaturation {
    const ID: u16 = CLUSTER_ID;
}

impl Command for EnhancedMoveToHueAndSaturation {
    const ID: u8 = 0x43;
    const DIRECTION: Direction = Direction::ClientToServer;
}
