use crate::Cluster;

/// Trait to get the cluster ID of an object.
pub trait ClusterIdAware {
    /// Return the cluster ID.
    #[must_use]
    fn cluster_id(&self) -> u16;
}

impl<T> ClusterIdAware for T
where
    T: Cluster,
{
    fn cluster_id(&self) -> u16 {
        T::ID
    }
}
