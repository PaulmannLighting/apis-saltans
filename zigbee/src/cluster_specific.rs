use crate::Cluster;
use crate::cluster_id::ClusterId;

/// Trait for cluster-specific commands.
pub trait ClusterSpecific {
    /// The cluster ID.
    const CLUSTER: ClusterId;
}

impl<T> Cluster for T
where
    T: ClusterSpecific,
{
    const ID: u16 = T::CLUSTER.as_u16();
}
