use zigbee::Direction;

pub use self::command_id::CommandId;
pub use self::customizable::Customizable;
pub use self::global::Global;
pub use self::native::Native;
use crate::Scope;

mod command_id;
mod customizable;
mod global;
mod native;

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
    const SCOPE: Scope;

    /// Whether to disable the default response for this command.
    const DISABLE_DEFAULT_RESPONSE: bool = false;
}
