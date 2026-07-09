pub use self::cluster_id::ClusterId;

mod cluster_id;

/// Trait to identify a Zigbee cluster.
pub trait Cluster<T = u16> {
    /// The cluster identifier.
    const ID: T;
}

impl<T> Cluster<u16> for T
where
    T: Cluster<ClusterId>,
{
    const ID: u16 = T::ID.as_u16();
}
