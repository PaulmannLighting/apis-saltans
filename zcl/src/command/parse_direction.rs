use zb_core::Direction;

/// Direction matching rule used when parsing incoming command frames.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ParseDirection {
    /// Accept only the specified direction.
    Single(Direction),

    /// Accept both client-to-server and server-to-client frames.
    Both,
}

impl ParseDirection {
    /// Return whether this rule accepts `direction`.
    #[must_use]
    pub const fn accepts(self, direction: Direction) -> bool {
        matches!(
            (self, direction),
            (Self::Both, _)
                | (
                    Self::Single(Direction::ClientToServer),
                    Direction::ClientToServer
                )
                | (
                    Self::Single(Direction::ServerToClient),
                    Direction::ServerToClient
                )
        )
    }
}
