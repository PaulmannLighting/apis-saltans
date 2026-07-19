use std::collections::BTreeSet;

use zb_core::Cluster;

/// Input and output cluster set advertised by one local application endpoint.
///
/// This compact helper stores only validated cluster IDs. The NCP endpoint APIs return full
/// `zb_zdp::SimpleDescriptor` values instead, since coordinator operations also require endpoint,
/// profile, device, and application-version metadata.
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
