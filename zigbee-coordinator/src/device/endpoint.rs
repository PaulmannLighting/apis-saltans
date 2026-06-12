use std::collections::BTreeMap;

use zigbee::Application;

pub use self::cluster::Cluster;

mod cluster;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct Endpoint {
    input_clusters: BTreeMap<u16, Cluster>,
    output_clusters: BTreeMap<u16, Cluster>,
}

impl Endpoint {
    /// Create a new endpoint.
    #[must_use]
    pub const fn new(
        input_clusters: BTreeMap<u16, Cluster>,
        output_clusters: BTreeMap<u16, Cluster>,
    ) -> Self {
        Self {
            input_clusters,
            output_clusters,
        }
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
