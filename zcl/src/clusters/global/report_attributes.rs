//! Report Attributes Command.

use zb_core::Direction;

pub use self::attribute_report::AttributeReport;
use crate::macros::zcl_command;
use crate::storage::Records;

mod attribute_report;

zcl_command! {
    /// Report Attributes Command.
    Command {
        Global;
        command_id: 0x0A;
        direction: Direction::ServerToClient;
        => crate::global::ReportAttributes;
        fields {
            reports: Records<AttributeReport>,
        }

        getters {
            /// Returns the attribute reports of the command.
            #[must_use]
            pub fn into_reports(self) -> Records<AttributeReport> {
                self.reports
            }
        }
    }
}
