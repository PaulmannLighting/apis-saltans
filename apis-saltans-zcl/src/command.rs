use apis_saltans_core::Direction;
use const_env::env_item;

pub use self::command_dispatch::CommandDispatch;
pub use self::parse_direction::ParseDirection;
pub use self::scoped::Scoped;

mod command_dispatch;
mod parse_direction;
mod scoped;

/// Trait to identify a Zigbee command.
pub trait Command {
    /// The command identifier.
    const ID: u8;

    /// The command direction.
    const DIRECTION: Direction;

    /// The command directions accepted when parsing incoming frames.
    const PARSE_DIRECTION: ParseDirection = ParseDirection::Single(Self::DIRECTION);

    /// Whether to disable the default response for this command.
    #[env_item("ZCL_DISABLE_DEFAULT_RESPONSE")]
    const DISABLE_DEFAULT_RESPONSE: bool = false;
}

/// Blanket implementation for boxed commands.
impl<T> Command for Box<T>
where
    T: Command,
{
    const ID: u8 = T::ID;
    const DIRECTION: Direction = T::DIRECTION;
    const PARSE_DIRECTION: ParseDirection = T::PARSE_DIRECTION;
    const DISABLE_DEFAULT_RESPONSE: bool = T::DISABLE_DEFAULT_RESPONSE;
}
