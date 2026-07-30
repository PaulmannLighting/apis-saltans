use core::ops::Not;

use num_enum::{IntoPrimitive, TryFromPrimitive};

/// Frame direction.
///
/// Applying the logical-not operator returns the opposite direction.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Clone, Copy, Debug, Eq, Hash, IntoPrimitive, Ord, PartialEq, PartialOrd, TryFromPrimitive,
)]
#[num_enum(error_type(name = u8, constructor = core::convert::identity))]
#[repr(u8)]
pub enum Direction {
    /// Frame is sent from the client side to the server side.
    ClientToServer = 0x00,

    /// Frame is sent from the server side to the client side.
    ServerToClient = 0x01,
}

impl Not for Direction {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::ClientToServer => Self::ServerToClient,
            Self::ServerToClient => Self::ClientToServer,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Direction;

    #[test]
    fn logical_not_inverts_direction() {
        assert_eq!(!Direction::ClientToServer, Direction::ServerToClient);
        assert_eq!(!Direction::ServerToClient, Direction::ClientToServer);
    }

    #[test]
    fn double_inversion_restores_direction() {
        let direction = Direction::ClientToServer;

        assert_eq!(!!direction, direction);
    }
}
