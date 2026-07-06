use apis_saltans_core::{Cluster, Endpoint, Profile};

/// APS metadata for a frame.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Metadata {
    cluster_id: u16,
    profile: Option<Profile>,
    source_endpoint: Option<Endpoint>,
}

impl Metadata {
    /// Create new APS metadata.
    #[must_use]
    pub const fn new(
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

    /// Create new APS metadata for the given cluster type.
    #[must_use]
    pub const fn for_cluster<T>() -> Self
    where
        T: Cluster,
    {
        Self {
            cluster_id: T::ID,
            profile: None,
            source_endpoint: None,
        }
    }

    /// Create new APS metadata for a ZDP command.
    #[must_use]
    pub const fn zdp(cluster_id: u16) -> Self {
        Self {
            cluster_id,
            profile: Some(Profile::Network),
            source_endpoint: Some(Endpoint::Data),
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
