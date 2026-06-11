use std::collections::BTreeMap;

use zigbee::Application;

pub use self::cluster::Cluster;

mod cluster;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Endpoint {
    id: Application,
    clusters: BTreeMap<u16, Cluster>,
}

impl Endpoint {
    /// Create a new endpoint.
    #[must_use]
    pub const fn new(id: Application, clusters: BTreeMap<u16, Cluster>) -> Self {
        Self { id, clusters }
    }

    #[must_use]
    pub const fn id(&self) -> Application {
        self.id
    }

    #[must_use]
    pub const fn clusters(&self) -> &BTreeMap<u16, Cluster> {
        &self.clusters
    }

    pub const fn clusters_mut(&mut self) -> &mut BTreeMap<u16, Cluster> {
        &mut self.clusters
    }
}
