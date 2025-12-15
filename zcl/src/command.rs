use zigbee::{Cluster, Direction};

use crate::Type;

/// Trait to identify a Zigbee command.
pub trait Command: Cluster {
    /// The command identifier.
    const ID: u8;

    /// The command direction.
    const DIRECTION: Direction;

    /// The command type.
    const TYPE: Type = Type::ClusterSpecific;

    /// Whether to disable the client response for this command.
    const DISABLE_CLIENT_RESPONSE: bool = true;

    /// The manufacturer code for this command, if any.
    const MANUFACTURER_CODE: Option<u16> = None;
}
