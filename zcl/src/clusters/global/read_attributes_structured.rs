//! Read Attributes Structured Command.

use std::boxed::Box;

use zb_core::Direction;

pub use self::record::Record;
use super::read_attributes::Response;
use crate::macros::zcl_command;

mod record;

zcl_command! {
    /// Read Attributes Structured Command.
    Command {
        Global;
        command_id: 0x0e;
        direction: Direction::ClientToServer;
        response: Response;
        => crate::global::ReadAttributesStructured;
        fields {
            records: Box<[Record]>,
        }

        getters {
            /// Return the structured attribute read records.
            #[must_use]
            pub fn records(&self) -> &[Record] {
                &self.records
            }
        }
    }
}
