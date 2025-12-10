use crate::cluster::Cluster;
use crate::direction::Direction;

/// Trait to identify a Zigbee command.
pub trait Command: Cluster {
    /// The command identifier.
    const ID: u8;

    /// The command direction.
    const DIRECTION: Direction;
}

/// Trait to identify a directed Zigbee command.
pub trait DirectedCommand: Command {
    /// A unique identifier for the command, combining command ID and direction.
    const ID: (u8, Direction) = (<Self as Command>::ID, <Self as Command>::DIRECTION);
}

impl<T> DirectedCommand for T where T: Command {}
