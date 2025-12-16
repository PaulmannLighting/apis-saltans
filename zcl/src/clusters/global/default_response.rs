//! Default Response Command.

use le_stream::{FromLeStream, ToLeStream};
use zigbee::Direction;

use crate::Global;

/// Default Response Command
#[derive(Clone, Debug, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
pub struct DefaultResponse {
    status: u8,
    command_id: u8,
}

impl DefaultResponse {
    /// Create a new `DefaultResponse` instance.
    #[must_use]
    pub const fn new(status: u8, command_id: u8) -> Self {
        Self { status, command_id }
    }

    /// Return the status of the default response.
    #[must_use]
    pub const fn status(&self) -> u8 {
        self.status
    }

    /// Return the command ID of the default response.
    #[must_use]
    pub const fn command_id(&self) -> u8 {
        self.command_id
    }
}

impl Global for DefaultResponse {
    const ID: u8 = 0x0b;
    const DIRECTION: Direction = Direction::ClientToServer;
}
