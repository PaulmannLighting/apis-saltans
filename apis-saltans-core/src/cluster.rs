pub use self::cluster_id::ClusterId;

mod cluster_id;

/// Trait to identify Zigbee zcl.
pub trait Cluster<T = u16> {
    /// The cluster identifier.
    const ID: T;
}

impl<T> Cluster<u16> for T
where
    T: Cluster<ClusterId>,
{
    const ID: u16 = <T as Cluster<ClusterId>>::ID.as_u16();
}
