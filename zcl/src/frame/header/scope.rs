use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

/// Command scope.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, FromPrimitive)]
#[repr(u8)]
pub enum Scope {
    /// A global command.
    Global = 0x00,
    /// A cluster-specific command.
    ClusterSpecific = 0x01,
}

impl TryFrom<u8> for Scope {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or(value)
    }
}
