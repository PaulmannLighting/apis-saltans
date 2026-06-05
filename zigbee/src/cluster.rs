pub use cluster_id::ClusterId;
pub use cluster_specific::ClusterSpecific;

mod cluster_id;
mod cluster_specific;

/// Trait to identify Zigbee zcl.
pub trait Cluster {
    /// The cluster identifier.
    const ID: u16;
}

impl<T> Cluster for T
where
    T: ClusterSpecific,
{
    const ID: u16 = T::CLUSTER.as_u16();
}
