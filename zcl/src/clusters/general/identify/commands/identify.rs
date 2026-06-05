use core::time::Duration;

use le_stream::{FromLeStream, ToLeStream};
use zigbee::types::Uint16;
use zigbee::{Cluster, Direction};

use crate::{ClusterId, Command, Native};

/// Toggle the identify state of a device.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
pub struct Identify {
    identify_time_secs: Uint16,
}

impl Identify {
    /// Create a new `Identify` command with the specified identify time.
    #[must_use]
    pub const fn new(identify_time_secs: Uint16) -> Self {
        Self { identify_time_secs }
    }

    /// Return the identify time seconds for this command.
    #[must_use]
    pub fn identify_time_secs(self) -> Option<u16> {
        self.identify_time_secs.into()
    }

    /// Return the identify time for this command.
    #[must_use]
    pub fn identify_time(self) -> Option<Duration> {
        self.identify_time_secs()
            .map(u64::from)
            .map(Duration::from_secs)
    }
}

impl Cluster for Identify {
    const ID: u16 = ClusterId::Identify.as_u16();
}

impl Command for Identify {
    const ID: u8 = 0x00;
    const DIRECTION: Direction = Direction::ClientToServer;
}

impl Native for Identify {}

impl From<Identify> for crate::Cluster {
    fn from(command: Identify) -> Self {
        Self::Identify(command.into())
    }
}
