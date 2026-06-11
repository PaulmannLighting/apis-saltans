//! Write Attributes Command and Response.

use alloc::boxed::Box;
use core::ops::Deref;

use le_stream::{FromLeStream, ToLeStream};
use zigbee::{Direction, ExpectResponse};

pub use self::record::Record;
pub use self::status::Status;
use crate::command::Scoped;
use crate::{Cluster, Scope, global};

mod record;
mod status;

/// Write Attributes Command.
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

impl crate::Command for Command {
    const ID: u8 = 0x02;
    const DIRECTION: Direction = Direction::ClientToServer;
}

impl Scoped for Command {
    const SCOPE: Scope = Scope::Global;
}

impl ExpectResponse<Cluster> for Command {
    type Response = Response;
}

impl From<Command> for Cluster {
    fn from(cmd: Command) -> Self {
        Self::Global(cmd.into())
    }
}

/// Write Attributes Command Response.
#[derive(Clone, Debug, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
pub struct Response {
    records: Box<[Status]>,
}

impl Response {
    /// Create a new write attributes command response.
    #[must_use]
    pub const fn new(records: Box<[Status]>) -> Self {
        Self { records }
    }
}

impl Deref for Response {
    type Target = [Status];

    fn deref(&self) -> &Self::Target {
        &self.records
    }
}

impl IntoIterator for Response {
    type Item = <Box<[Status]> as IntoIterator>::Item;
    type IntoIter = <Box<[Status]> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.records.into_iter()
    }
}

impl crate::Command for Response {
    const ID: u8 = 0x04;
    const DIRECTION: Direction = Direction::ServerToClient;
}

impl Scoped for Response {
    const SCOPE: Scope = Scope::Global;
}

impl TryFrom<Cluster> for Response {
    type Error = Cluster;

    fn try_from(value: Cluster) -> Result<Self, Self::Error> {
        if let Cluster::Global(global::Command::WriteAttributesResponse(cmd)) = value {
            Ok(cmd)
        } else {
            Err(value)
        }
    }
}
