//! Read Reporting Configuration Command and Response.

use std::boxed::Box;

use zb_core::Direction;

pub use self::record::Record;
pub use self::status::Status;
use crate::macros::zcl_command;

mod record;
mod status;

zcl_command! {
    /// Read Reporting Configuration Command.
    Command {
        Global;
        command_id: 0x08;
        direction: Direction::ClientToServer;
        response: Response;
        => crate::global::ReadReportingConfiguration;
        fields {
            records: Box<[Record]>,
        }

        getters {
            /// Return the requested reporting-configuration records.
            #[must_use]
            pub fn records(&self) -> &[Record] {
                &self.records
            }
        }
    }
}

zcl_command! {
    /// Read Reporting Configuration Response Command.
    Response {
        Global;
        command_id: 0x09;
        direction: Direction::ServerToClient;
        => crate::global::ReadReportingConfigurationResponse;
        fields {
            records: Box<[Status]>,
        }

        getters {
            /// Return the reporting-configuration status records.
            #[must_use]
            pub fn records(&self) -> &[Status] {
                &self.records
            }
        }
    }
}
