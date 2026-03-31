use zigbee::Direction;

pub use self::command_id::CommandId;
pub use self::customizable::Customizable;
pub use self::global::Global;
pub use self::native::Native;
pub use self::scoped::Scoped;

mod command_id;
mod customizable;
mod global;
mod native;
mod scoped;

/// Trait to identify a Zigbee command.
pub trait Command {
    /// The command identifier.
    const ID: u8;

    /// The command direction.
    const DIRECTION: Direction;

    /// Whether to disable the default response for this command.
    const DISABLE_DEFAULT_RESPONSE: bool = false;
}
