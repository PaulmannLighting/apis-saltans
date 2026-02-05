use core::num::TryFromIntError;
use core::time::Duration;

use le_stream::{FromLeStream, ToLeStream};
use zigbee::{Cluster, Direction, FromDeciSeconds, IntoDeciSeconds};

pub use self::on_off_control::OnOffControl;
use crate::Command;
use crate::general::on_off::CLUSTER_ID;

mod on_off_control;

/// Command to turn on a device for a specified time, then turn it off after a wait period.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
pub struct OnWithTimedOff {
    on_off_control: OnOffControl,
    on_time: u16,
    off_wait_time: u16,
}

impl OnWithTimedOff {
    /// Create a new `OnWithTimedOff` command.
    #[must_use]
    pub const fn new(on_off_control: OnOffControl, on_time: u16, off_wait_time: u16) -> Self {
        Self {
            on_off_control,
            on_time,
            off_wait_time,
        }
    }

    /// Create a new `OnWithTimedOff` command with durations.
    pub fn try_new(
        on_off_control: OnOffControl,
        on_time: Duration,
        off_wait_time: Duration,
    ) -> Result<Self, TryFromIntError> {
        Ok(Self::new(
            on_off_control,
            on_time.into_deci_seconds().try_into()?,
            off_wait_time.into_deci_seconds().try_into()?,
        ))
    }

    /// Return the on/off control field.
    #[must_use]
    pub const fn on_off_control(&self) -> OnOffControl {
        self.on_off_control
    }

    /// Return the on time in seconds.
    #[must_use]
    pub fn on_time(&self) -> Duration {
        Duration::from_deci_seconds(self.on_time)
    }

    /// Return the off wait time in seconds.
    #[must_use]
    pub fn off_wait_time(&self) -> Duration {
        Duration::from_deci_seconds(self.off_wait_time)
    }
}

impl Cluster for OnWithTimedOff {
    const ID: u16 = CLUSTER_ID;
}

impl Command for OnWithTimedOff {
    const ID: u8 = 0x42;
    const DIRECTION: Direction = Direction::ClientToServer;
}
