/// Frame direction.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(u8)]
pub enum Direction {
    /// Frame is sent from client side to server side.
    ClientToServer = 0x00,
    /// Frame is sent from server side to client side.
    ServerToClient = 0x01,
}
