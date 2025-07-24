use core::time::Duration;

use crate::zcl::identify::CLUSTER_ID;
use crate::zcl::{Cluster, Command};

/// Toggle the identify state of a device.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Identify {
    identify_time_secs: u16,
}

impl Identify {
    /// Create a new `Identify` command with the specified identify time.
    #[must_use]
    pub const fn new(identify_time_secs: u16) -> Self {
        Self { identify_time_secs }
    }

    /// Return the identify time seconds for this command.
    #[must_use]
    pub const fn identify_time_secs(self) -> u16 {
        self.identify_time_secs
    }

    /// Return the identify time for this command.
    #[must_use]
    pub fn identify_time(self) -> Duration {
        Duration::from_secs(u64::from(self.identify_time_secs))
    }
}

impl Cluster for Identify {
    const ID: u16 = CLUSTER_ID;
}

impl Command for Identify {
    const ID: u8 = 0x00;
}
