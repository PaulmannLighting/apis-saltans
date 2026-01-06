use zigbee::{Endpoint, Profile};

/// APS metadata for a aps.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Metadata {
    cluster_id: u16,
    profile: Option<Profile>,
    source_endpoint: Option<Endpoint>,
}

impl Metadata {
    /// Create new APS metadata.
    #[must_use]
    pub(crate) const fn new(
        cluster_id: u16,
        profile: Option<Profile>,
        source_endpoint: Option<Endpoint>,
    ) -> Self {
        Self {
            cluster_id,
            profile,
            source_endpoint,
        }
    }

    /// Return the cluster ID.
    #[must_use]
    pub const fn cluster_id(&self) -> u16 {
        self.cluster_id
    }

    /// Return the profile ID.
    #[must_use]
    pub const fn profile(&self) -> Option<Profile> {
        self.profile
    }

    /// Return the source endpoint.
    #[must_use]
    pub const fn source_endpoint(&self) -> Option<Endpoint> {
        self.source_endpoint
    }
}
