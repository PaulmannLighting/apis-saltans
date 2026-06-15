use std::collections::btree_map::OccupiedEntry;
use std::collections::{BTreeMap, BTreeSet};

use zigbee::{Address, ClusterId, Endpoint};

use super::BIND_OUTPUT_CLUSTERS;
use crate::discovery::EndpointInfo;

/// Type alias for the device map.
pub type Devices = BTreeMap<Address, BTreeMap<Endpoint, (EndpointInfo, BTreeSet<ClusterId>)>>;

/// Helper trait to operate on the device map.
pub trait DevicesExt {
    /// Update the device map.
    fn update(
        &mut self,
        address: Address,
        endpoints: BTreeMap<Endpoint, EndpointInfo>,
    ) -> OccupiedEntry<'_, Address, BTreeMap<Endpoint, (EndpointInfo, BTreeSet<ClusterId>)>>;

    /// Set an endpoint as bound to a cluster.
    fn endpoint_bound(&mut self, address: &Address, endpoint: Endpoint, cluster: ClusterId);

    /// Remove a device if all its endpoints are bound.
    fn remove_if_binding_complete(
        &mut self,
        address: &Address,
    ) -> Option<BTreeMap<Endpoint, EndpointInfo>>;
}

impl DevicesExt for Devices {
    fn update(
        &mut self,
        address: Address,
        endpoints: BTreeMap<Endpoint, EndpointInfo>,
    ) -> OccupiedEntry<'_, Address, BTreeMap<Endpoint, (EndpointInfo, BTreeSet<ClusterId>)>> {
        self.entry(address).insert_entry(
            endpoints
                .into_iter()
                .map(|(endpoint, info)| (endpoint, (info, BTreeSet::new())))
                .collect(),
        )
    }

    fn endpoint_bound(&mut self, address: &Address, endpoint: Endpoint, cluster: ClusterId) {
        let Some(endpoints) = self.get_mut(address) else {
            return;
        };

        let Some((_, clusters)) = endpoints.get_mut(&endpoint) else {
            return;
        };

        clusters.insert(cluster);
    }

    fn remove_if_binding_complete(
        &mut self,
        address: &Address,
    ) -> Option<BTreeMap<Endpoint, EndpointInfo>> {
        let endpoints = self.get(address)?;

        if endpoints
            .iter()
            .all(|(endpoint, (endpoint_info, bound_clusters))| {
                is_bound(*endpoint, endpoint_info, bound_clusters)
            })
        {
            let endpoints = self
                .remove(address)?
                .into_iter()
                .map(|(endpoint, (endpoint_info, _))| (endpoint, endpoint_info))
                .collect();
            return Some(endpoints);
        }

        None
    }
}

/// Check if an endpoint is bound to all clusters that we want to bind.
fn is_bound(
    endpoint: Endpoint,
    endpoint_info: &EndpointInfo,
    bound_clusters: &BTreeSet<ClusterId>,
) -> bool {
    // Endpoint is an application endpoint ...
    let Endpoint::Application(_) = endpoint else {
        return false;
    };

    // ... and all clusters that we want to bind ...
    let clusters_to_be_bound: BTreeSet<_> = BIND_OUTPUT_CLUSTERS
        .iter()
        .copied()
        .filter(|cluster| {
            endpoint_info
                .descriptor()
                .output_clusters()
                .contains(&cluster.as_u16())
        })
        .collect();

    // ... are bound.
    bound_clusters == &clusters_to_be_bound
}
