use std::collections::BTreeMap;

use zigbee::types::Type;

/// State of a cluster.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Cluster {
    id: u16,
    bound: bool,
    attributes: BTreeMap<u16, Type>,
}

impl Cluster {
    /// Create a new cluster.
    #[must_use]
    pub const fn new(id: u16) -> Self {
        Self {
            id,
            bound: false,
            attributes: BTreeMap::new(),
        }
    }

    #[must_use]
    pub const fn id(&self) -> u16 {
        self.id
    }

    #[must_use]
    pub const fn bound(&self) -> bool {
        self.bound
    }

    #[must_use]
    pub const fn attributes(&self) -> &BTreeMap<u16, Type> {
        &self.attributes
    }

    pub const fn attributes_mut(&mut self) -> &mut BTreeMap<u16, Type> {
        &mut self.attributes
    }

    pub const fn bind(&mut self) {
        self.bound = true;
    }

    pub const fn unbind(&mut self) {
        self.bound = false;
    }
}
