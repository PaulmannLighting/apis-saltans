/// Trait to identify Zigbee zcl.
pub trait Cluster {
    /// The cluster identifier.
    const ID: u16;
}

/// Trait to get the cluster ID of an object.
pub trait ClusterId {
    /// Return the cluster ID.
    #[must_use]
    fn cluster_id(&self) -> u16;
}

impl<T> ClusterId for T
where
    T: Cluster,
{
    fn cluster_id(&self) -> u16 {
        T::ID
    }
}
