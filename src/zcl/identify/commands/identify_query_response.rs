use core::time::Duration;

use zigbee::types::Uint16;

use crate::zcl::identify::CLUSTER_ID;
use crate::{Cluster, Command};

/// Response to the [`IdentifyQuery`](crate::zcl::identify::IdentifyQuery) command.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct IdentifyQueryResponse {
    timeout_secs: Uint16,
}

impl IdentifyQueryResponse {
    /// Create a new `IdentifyQueryResponse` command with the specified timeout.
    #[must_use]
    pub const fn new(timeout_secs: Uint16) -> Self {
        Self { timeout_secs }
    }

    /// Return the identify time in seconds.
    #[must_use]
    pub fn timeout_secs(self) -> Option<u16> {
        self.timeout_secs.into()
    }

    /// Return the identify timeout for this command.
    #[must_use]
    pub fn timeout(self) -> Option<Duration> {
        self.timeout_secs().map(u64::from).map(Duration::from_secs)
    }
}

impl Cluster for IdentifyQueryResponse {
    const ID: u16 = CLUSTER_ID;
}

impl Command for IdentifyQueryResponse {
    const ID: u8 = 0x00;
}
