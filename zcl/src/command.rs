use zigbee::Direction;

pub use self::command_dispatch::CommandDispatch;
pub use self::scoped::Scoped;

mod command_dispatch;
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
