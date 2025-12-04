use crate::cluster::Cluster;
use crate::direction::Direction;

/// Trait to identify a Zigbee command.
pub trait Command: Cluster {
    /// The command identifier.
    const ID: u8;

    /// The command direction.
    const DIRECTION: Direction;
}
