//! Write Attributes Undivided Command.

use core::ops::Deref;
use std::boxed::Box;

use apis_saltans_core::Direction;

pub use super::write_attributes::{Record, Response, Status};
use crate::macros::zcl_command;

zcl_command! {
    /// Write Attributes Undivided Command.
    Command {
        Global;
        command_id: 0x03;
        direction: Direction::ClientToServer;
        response: Response;
        => crate::global::WriteAttributesUndivided(box);
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
