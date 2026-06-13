use std::collections::BTreeMap;

use zigbee::types::Type;

/// State of a cluster.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Cluster {
    bound: bool,
    attributes: BTreeMap<u16, Type>,
}

impl Cluster {
    /// Create a new cluster.
    #[must_use]
    pub const fn new(bound: bool, attributes: BTreeMap<u16, Type>) -> Self {
        Self { bound, attributes }
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
