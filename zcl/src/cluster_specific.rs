use crate::ClusterId;

/// Trait for cluster-specific commands.
pub trait ClusterSpecific {
    /// The cluster ID.
    const CLUSTER: ClusterId;
}
