//! A request to read a set of raw attributes.

use alloc::collections::BTreeSet;

/// A request to read a set of raw attributes.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct RawAttributes {
    cluster: u16,
    manufacturer_code: Option<u16>,
    ids: BTreeSet<u16>,
}

impl RawAttributes {
    /// Create a new raw attributes request.
    #[must_use]
    pub const fn new(cluster: u16, manufacturer_code: Option<u16>, ids: BTreeSet<u16>) -> Self {
        Self {
            cluster,
            manufacturer_code,
            ids,
        }
    }

    /// Return the cluster ID.
    #[must_use]
    pub const fn cluster(&self) -> u16 {
        self.cluster
    }

    /// Return the manufacturer code.
    #[must_use]
    pub const fn manufacturer_code(&self) -> Option<u16> {
        self.manufacturer_code
    }

    /// Return a reference to the set of attribute IDs.
    #[must_use]
    pub const fn ids(&self) -> &BTreeSet<u16> {
        &self.ids
    }
}
