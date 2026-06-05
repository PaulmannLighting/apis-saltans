use crate::cluster::cluster_id::ClusterId;

/// Trait for cluster-specific commands.
pub trait ClusterSpecific {
    /// The cluster ID.
    const CLUSTER: ClusterId;
}
