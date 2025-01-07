use crate::zcl::cluster::Cluster;

/// Trait to identify a Zigbee command.
pub trait Command: Cluster {
    /// The command identifier.
    const ID: u8;
}
