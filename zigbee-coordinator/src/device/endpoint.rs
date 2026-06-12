use std::collections::BTreeMap;

use zigbee::Application;

pub use self::cluster::Cluster;

mod cluster;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Endpoint {
    id: Application,
    input_clusters: BTreeMap<u16, Cluster>,
    output_clusters: BTreeMap<u16, Cluster>,
}

impl Endpoint {
    /// Create a new endpoint.
    #[must_use]
    pub const fn new(id: Application) -> Self {
        Self {
            id,
            input_clusters: BTreeMap::new(),
            output_clusters: BTreeMap::new(),
        }
    }

    #[must_use]
    pub const fn id(&self) -> Application {
        self.id
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
