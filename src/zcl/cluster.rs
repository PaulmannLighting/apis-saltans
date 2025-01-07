/// Trait to identify Zigbee clusters.
pub trait Cluster {
    /// The cluster identifier.
    const ID: u16;
}
