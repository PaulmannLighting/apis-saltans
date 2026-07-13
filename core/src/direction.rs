use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

/// Frame direction.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, FromPrimitive)]
#[repr(u8)]
pub enum Direction {
    /// Frame is sent from the client side to the server side.
    ClientToServer = 0x00,

    /// Frame is sent from the server side to the client side.
    ServerToClient = 0x01,
}

impl TryFrom<u8> for Direction {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or(value)
    }
}
