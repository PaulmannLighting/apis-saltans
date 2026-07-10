//! Write Attributes Structured Command and Response.

use std::boxed::Box;

use apis_saltans_core::Direction;

pub use self::record::Record;
pub use self::status::Status;
use crate::macros::zcl_command;

mod record;
mod status;

zcl_command! {
    /// Write Attributes Structured Command.
    Command {
        Global;
        command_id: 0x0f;
        direction: Direction::ClientToServer;
        response: Response;
        => crate::global::WriteAttributesStructured;
        fields {
            records: Box<[Record]>,
        }

        getters {
            /// Return the structured write attribute records.
            #[must_use]
            pub fn records(&self) -> &[Record] {
                &self.records
            }
        }
    }
}

zcl_command! {
    /// Write Attributes Structured Response Command.
    Response {
        Global;
        command_id: 0x10;
        direction: Direction::ServerToClient;
        => crate::global::WriteAttributesStructuredResponse;
        fields {
            records: Box<[Status]>,
        }

        getters {
            /// Return the structured write status records.
            #[must_use]
            pub fn records(&self) -> &[Status] {
                &self.records
            }
        }
    }
}
