use zigbee::Direction;

pub use self::cluster_directed::ClusterDirected;
pub use self::global::Global;
use crate::Scope;

mod cluster_directed;
mod global;

/// Trait to identify a Zigbee command.
pub trait Command {
    /// The command identifier.
    const ID: u8;

    /// The command direction.
    const DIRECTION: Direction;

    /// The command scope.
    ///
    /// `Scope::Global` commands can be sent to any cluster, while
    /// `Scope::ClusterSpecific` commands are specific to a particular cluster.
    ///
    /// Default is `Scope::ClusterSpecific`.
    const SCOPE: Scope = Scope::ClusterSpecific;

    /// Whether to disable the client response for this command.
    const DISABLE_CLIENT_RESPONSE: bool = false;

    /// The manufacturer code for this command, if any.
    const MANUFACTURER_CODE: Option<u16> = None;
}
