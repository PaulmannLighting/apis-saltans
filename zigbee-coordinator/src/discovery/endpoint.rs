use std::collections::BTreeMap;

use zdp::SimpleDescriptor;

pub use self::cluster::{Attributes, Cluster};

mod cluster;

#[derive(Debug)]
pub struct Endpoint {
    profile_id: u16,
    device_id: u16,
    version: u8,
    input_clusters: BTreeMap<u16, Cluster>,
    output_clusters: BTreeMap<u16, Cluster>,
}

impl Endpoint {
    /// Create a new endpoint.
    #[must_use]
    pub const fn new(
        profile_id: u16,
        device_id: u16,
        version: u8,
        input_clusters: BTreeMap<u16, Cluster>,
        output_clusters: BTreeMap<u16, Cluster>,
    ) -> Self {
        Self {
            profile_id,
            device_id,
            version,
            input_clusters,
            output_clusters,
        }
    }

    #[must_use]
    pub const fn profile_id(&self) -> u16 {
        self.profile_id
    }

    #[must_use]
    pub const fn device_id(&self) -> u16 {
        self.device_id
    }

    #[must_use]
    pub const fn version(&self) -> u8 {
        self.version
    }

    #[must_use]
    pub const fn input_clusters(&self) -> &BTreeMap<u16, Cluster> {
        &self.input_clusters
    }

    pub const fn input_clusters_mut(&mut self) -> &mut BTreeMap<u16, Cluster> {
        &mut self.input_clusters
    }

    #[must_use]
    pub const fn output_clusters(&self) -> &BTreeMap<u16, Cluster> {
        &self.output_clusters
    }

    pub const fn output_clusters_mut(&mut self) -> &mut BTreeMap<u16, Cluster> {
        &mut self.output_clusters
    }
}

impl From<SimpleDescriptor> for Endpoint {
    fn from(desc: SimpleDescriptor) -> Self {
        let (_, profile_id, device_id, version, input_clusters, output_clusters) =
            desc.into_parts();
        Self {
            profile_id,
            device_id,
            version,
            input_clusters: input_clusters
                .into_iter()
                .map(|id| (id, Cluster::default()))
                .collect(),
            output_clusters: output_clusters
                .into_iter()
                .map(|id| (id, Cluster::default()))
                .collect(),
        }
    }
}
