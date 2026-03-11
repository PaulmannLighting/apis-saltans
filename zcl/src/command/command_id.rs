use crate::Command;

/// Trait to get the command identifier.
pub trait CommandId {
    /// Return the command identifier.
    fn command_id(&self) -> u8;
}

impl<T> CommandId for T
where
    T: Command,
{
    fn command_id(&self) -> u8 {
        T::ID
    }
}
