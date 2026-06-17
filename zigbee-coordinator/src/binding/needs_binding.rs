use std::collections::BTreeMap;

use zigbee::{ClusterId, Endpoint};
use zigbee_persistence::Endpoint as EndpointInfo;

/// Helper trait to check if an endpoint needs binding.
pub trait NeedsBinding {
    /// Return `true` if any of the endpoints need binding.
    fn needs_binding(&self, clusters: &[ClusterId]) -> bool;
}

impl NeedsBinding for BTreeMap<Endpoint, EndpointInfo> {
    fn needs_binding(&self, clusters: &[ClusterId]) -> bool {
        self.values()
            .any(|endpoint_info| endpoint_info.needs_binding(clusters))
    }
}

impl NeedsBinding for EndpointInfo {
    fn needs_binding(&self, clusters: &[ClusterId]) -> bool {
        clusters.iter().any(|&cluster| {
            self.descriptor()
                .output_clusters()
                .contains(&cluster.as_u16())
        })
    }
}
