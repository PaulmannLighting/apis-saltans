use apis_saltans_core::{Cluster, Profile};

/// APS metadata for a frame.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Metadata {
    cluster_id: u16,
    profile: Profile,
}

impl Metadata {
    /// Create new APS metadata.
    #[must_use]
    pub const fn new(cluster_id: u16, profile: Profile) -> Self {
        Self {
            cluster_id,
            profile,
        }
    }

    /// Create new APS metadata for the given cluster type.
    #[must_use]
    pub const fn cluster<T>() -> Self
    where
        T: Cluster,
    {
        Self {
            cluster_id: T::ID,
            profile: T::PROFILE,
        }
    }

    /// Return the cluster ID.
    #[must_use]
    pub const fn cluster_id(&self) -> u16 {
        self.cluster_id
    }

    /// Return the profile ID.
    #[must_use]
    pub const fn profile(&self) -> Profile {
        self.profile
    }
}
