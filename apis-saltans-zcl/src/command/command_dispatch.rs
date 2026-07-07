use apis_saltans_core::Direction;

use crate::command::Scoped;
use crate::{Command, Scope};

/// Trait to get the command identifier.
pub trait CommandDispatch {
    /// Return the command identifier.
    fn command_id(&self) -> u8;

    /// Return the command scope.
    fn scope(&self) -> Scope;

    /// Return the command direction.
    fn direction(&self) -> Direction;

    /// Whether to disable the default response for outgoing command frames.
    ///
    /// For commands that do not define this value explicitly, this follows the
    /// compile-time `ZCL_DISABLE_DEFAULT_RESPONSE` switch.
    fn disable_default_response(&self) -> bool;
}

impl<T> CommandDispatch for T
where
    T: Command + Scoped,
{
    fn command_id(&self) -> u8 {
        T::ID
    }

    fn scope(&self) -> Scope {
        T::SCOPE
    }

    fn direction(&self) -> Direction {
        T::DIRECTION
    }

    fn disable_default_response(&self) -> bool {
        T::DISABLE_DEFAULT_RESPONSE
    }
}
