//! Write Attributes Command and Response.

use core::ops::Deref;
use std::boxed::Box;

use apis_saltans_core::Direction;

pub use self::record::Record;
pub use self::status::Status;
use crate::macros::zcl_command;

mod record;
mod status;

zcl_command! {
    /// Write Attributes Command.
    Command {
        Global;
        command_id: 0x02;
        direction: Direction::ClientToServer;
        response: Response;
        => crate::global::WriteAttributes(box);
        fields {
            records: Box<[Record]>,
        }
    }
}

impl Deref for Command {
    type Target = [Record];

    fn deref(&self) -> &Self::Target {
        &self.records
    }
}

zcl_command! {
    /// Write Attributes Command Response.
    Response {
        Global;
        command_id: 0x04;
        direction: Direction::ServerToClient;
        => crate::global::WriteAttributesResponse(box);
        fields {
            records: Box<[Status]>,
        }
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
