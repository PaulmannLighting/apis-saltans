use std::collections::BTreeSet;

use zb_core::Cluster;

/// Input and output cluster set advertised by one local application endpoint.
///
/// `Clusters` is the hardware-layer summary returned by `get_endpoints()`. It intentionally stores
/// only the cluster IDs exposed by the local endpoint; higher layers can combine these sets with
/// application endpoint IDs, profile IDs, device IDs, and version information when they need to
/// build full ZDP simple descriptors.
pub struct Clusters {
    input: BTreeSet<Cluster>,
    output: BTreeSet<Cluster>,
}

impl Clusters {
    /// Create a cluster summary from input and output cluster sets.
    #[must_use]
    pub const fn new(input: BTreeSet<Cluster>, output: BTreeSet<Cluster>) -> Self {
        Self { input, output }
    }

    /// Return the input clusters supported by the endpoint.
    #[must_use]
    pub const fn input(&self) -> &BTreeSet<Cluster> {
        &self.input
    }

    /// Return the output clusters supported by the endpoint.
    #[must_use]
    pub const fn output(&self) -> &BTreeSet<Cluster> {
        &self.output
    }
}
