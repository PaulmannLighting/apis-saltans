use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Display;

use apis_saltans_core::node::Descriptor;
use apis_saltans_core::{Address, ClusterId, Endpoint};

/// Device information.
#[derive(Debug)]
pub struct Device {
    pub address: Address,
    pub descriptor: Descriptor,
    pub endpoints: BTreeMap<Endpoint, EndpointInfo>,
}

impl Device {
    /// Yield clusters of endpoints that still need binding.
    pub fn clusters_to_bind(
        &self,
        clusters: &[ClusterId],
    ) -> impl Iterator<Item = (Endpoint, ClusterId)> {
        self.endpoints.iter().flat_map(|(endpoint, info)| {
            info.clusters_to_bind(clusters)
                .map(|cluster| (*endpoint, cluster))
        })
    }

    /// Return true if any of the clusters still need binding.
    #[must_use]
    pub fn needs_binding(&self, clusters: &[ClusterId]) -> bool {
        self.clusters_to_bind(clusters).next().is_some()
    }
}

impl Display for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.address.fmt(f)
    }
}

impl From<crate::Device> for Device {
    fn from(value: crate::Device) -> Self {
        Self {
            address: value.address,
            descriptor: value.descriptor,
            endpoints: value
                .endpoints
                .into_iter()
                .map(|(endpoint, info)| (endpoint, info.into()))
                .collect(),
        }
    }
}

impl From<Device> for crate::Device {
    fn from(value: Device) -> Self {
        Self {
            address: value.address,
            descriptor: value.descriptor,
            endpoints: value
                .endpoints
                .into_iter()
                .map(|(endpoint, info)| (endpoint, info.into()))
                .collect(),
        }
    }
}

#[derive(Debug)]
pub struct EndpointInfo {
    pub info: crate::Endpoint,
    pub bound_clusters: BTreeSet<ClusterId>,
}

impl EndpointInfo {
    /// Yield clusters that still need binding.
    pub fn clusters_to_bind(&self, clusters: &[ClusterId]) -> impl Iterator<Item = ClusterId> {
        clusters
            .iter()
            .copied()
            .filter(|cluster| {
                self.info
                    .descriptor()
                    .output_clusters()
                    .contains(&cluster.as_u16())
            })
            .filter(|cluster| !self.bound_clusters.contains(cluster))
    }

    /// Return true if any of the clusters still need binding.
    #[must_use]
    pub fn needs_binding(&self, clusters: &[ClusterId]) -> bool {
        self.clusters_to_bind(clusters).next().is_some()
    }
}

impl From<crate::Endpoint> for EndpointInfo {
    fn from(value: crate::Endpoint) -> Self {
        Self {
            info: value,
            bound_clusters: BTreeSet::new(),
        }
    }
}

impl From<EndpointInfo> for crate::Endpoint {
    fn from(value: EndpointInfo) -> Self {
        value.info
    }
}
