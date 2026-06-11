//! Write Attributes Undivided Command.

use alloc::boxed::Box;
use core::ops::Deref;

use zigbee::Direction;

pub use super::write_attributes::{Record, Response, Status};

/// Write Attributes Undivided Command.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Command {
    records: Box<[Record]>,
}

impl Command {
    /// Create a new command.
    #[must_use]
    pub const fn new(records: Box<[Record]>) -> Self {
        Self { records }
    }
}

impl Deref for Command {
    type Target = [Record];

    fn deref(&self) -> &Self::Target {
        &self.records
    }
}

impl crate::Command for Command {
    const ID: u8 = 0x03;
    const DIRECTION: Direction = Direction::ClientToServer;
}
