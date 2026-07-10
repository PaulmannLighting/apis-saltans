use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Display;

use apis_saltans_core::node::Descriptor;
use apis_saltans_core::{Cluster, Endpoint, FullAddress};

use crate::discovery::OutgoingDevice;

/// Device information.
#[derive(Debug)]
pub struct Device {
    pub address: FullAddress,
    pub descriptor: Descriptor,
    pub endpoints: BTreeMap<Endpoint, EndpointInfo>,
}

impl Device {
    /// Yield clusters of endpoints that still need binding.
    pub fn clusters_to_bind(
        &self,
        clusters: &[Cluster],
    ) -> impl Iterator<Item = (Endpoint, Cluster)> {
        self.endpoints.iter().flat_map(|(endpoint, info)| {
            info.clusters_to_bind(clusters)
                .map(|cluster| (*endpoint, cluster))
        })
    }

    /// Return true if any of the clusters still need binding.
    #[must_use]
    pub fn needs_binding(&self, clusters: &[Cluster]) -> bool {
        self.clusters_to_bind(clusters).next().is_some()
    }
}

impl Display for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.address.fmt(f)
    }
}

impl From<OutgoingDevice> for Device {
    fn from(device: OutgoingDevice) -> Self {
        Self {
            address: device.address,
            descriptor: device.descriptor,
            endpoints: device
                .endpoints
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        EndpointInfo {
                            info: v.into(),
                            bound_clusters: BTreeSet::new(),
                        },
                    )
                })
                .collect(),
        }
    }
}

impl From<Device> for crate::Device {
    fn from(device: Device) -> Self {
        Self {
            descriptor: device.descriptor,
            endpoints: device
                .endpoints
                .into_iter()
                .map(|(k, v)| (k, v.info))
                .collect(),
        }
    }
}

#[derive(Debug)]
pub struct EndpointInfo {
    pub info: crate::EndpointInfo,
    pub bound_clusters: BTreeSet<Cluster>,
}

impl EndpointInfo {
    /// Yield clusters that still need binding.
    pub fn clusters_to_bind(&self, clusters: &[Cluster]) -> impl Iterator<Item = Cluster> {
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
    pub fn needs_binding(&self, clusters: &[Cluster]) -> bool {
        self.clusters_to_bind(clusters).next().is_some()
    }
}

impl From<crate::EndpointInfo> for EndpointInfo {
    fn from(value: crate::EndpointInfo) -> Self {
        Self {
            info: value,
            bound_clusters: BTreeSet::new(),
        }
    }
}

impl From<EndpointInfo> for crate::EndpointInfo {
    fn from(value: EndpointInfo) -> Self {
        value.info
    }
}
