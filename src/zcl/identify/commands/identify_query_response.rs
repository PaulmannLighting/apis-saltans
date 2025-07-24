use core::time::Duration;

use crate::zcl::identify::CLUSTER_ID;
use crate::zcl::{Cluster, Command};

/// Response to the [`IdentifyQuery`](crate::zcl::identify::IdentifyQuery) command.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct IdentifyQueryResponse {
    timeout_secs: u16,
}

impl IdentifyQueryResponse {
    /// Create a new `IdentifyQueryResponse` command with the specified timeout.
    #[must_use]
    pub const fn new(timeout_secs: u16) -> Self {
        Self { timeout_secs }
    }

    /// Return the identify timeout for this command.
    #[must_use]
    pub fn timeout(self) -> Duration {
        Duration::from_secs(u64::from(self.timeout_secs))
    }
}

impl Cluster for IdentifyQueryResponse {
    const ID: u16 = CLUSTER_ID;
}

impl Command for IdentifyQueryResponse {
    const ID: u8 = 0x00;
}
