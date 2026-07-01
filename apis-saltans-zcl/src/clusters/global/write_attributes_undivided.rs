//! Write Attributes Undivided Command.

use core::ops::Deref;
use std::boxed::Box;

use apis_saltans_core::{Direction, ExpectResponse};
use le_stream::{FromLeStream, ToLeStream};

pub use super::write_attributes::{Record, Response, Status};
use crate::command::Scoped;
use crate::{Cluster, Scope};

/// Write Attributes Undivided Command.
#[derive(Clone, Debug, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
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

impl From<Command> for Cluster {
    fn from(cmd: Command) -> Self {
        Self::Global(cmd.into())
    }
}

impl crate::Command for Command {
    const ID: u8 = 0x03;
    const DIRECTION: Direction = Direction::ClientToServer;
}

impl Scoped for Command {
    const SCOPE: Scope = Scope::Global;
}

impl ExpectResponse<Cluster> for Command {
    type Response = Response;
}
