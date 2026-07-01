/// Frame direction.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(u8)]
pub enum Direction {
    /// Frame is sent from the client side to the server side.
    ClientToServer = 0x00,

    /// Frame is sent from the server side to the client side.
    ServerToClient = 0x01,
}
