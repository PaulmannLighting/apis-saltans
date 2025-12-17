use zigbee::Endpoint;

/// APS metadata for a frame.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ApsMetadata {
    cluster_id: u16,
    profile_id: Option<u16>,
    source_endpoint: Option<Endpoint>,
}

impl ApsMetadata {
    /// Create new APS metadata.
    #[must_use]
    pub(crate) fn new(
        cluster_id: u16,
        profile_id: Option<u16>,
        source_endpoint: Option<Endpoint>,
    ) -> Self {
        Self {
            cluster_id,
            profile_id,
            source_endpoint,
        }
    }

    /// Return the cluster ID.
    #[must_use]
    pub fn cluster_id(&self) -> u16 {
        self.cluster_id
    }

    /// Return the profile ID.
    #[must_use]
    pub fn profile_id(&self) -> Option<u16> {
        self.profile_id
    }

    /// Return the source endpoint.
    #[must_use]
    pub fn source_endpoint(&self) -> Option<Endpoint> {
        self.source_endpoint
    }
}
