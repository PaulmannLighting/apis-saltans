//! Report Attributes Command.

use std::boxed::Box;

use apis_saltans_core::Direction;

pub use self::attribute_report::AttributeReport;
use crate::macros::zcl_command;

mod attribute_report;

zcl_command! {
    /// Report Attributes Command.
    Command {
        Global;
        command_id: 0x0A;
        direction: Direction::ServerToClient;
        => crate::global::ReportAttributes;
        fields {
            reports: Box<[AttributeReport]>,
        }

        getters {
            /// Returns the attribute reports of the command.
            #[must_use]
            pub fn reports(&self) -> &[AttributeReport] {
                &self.reports
            }
        }
    }
}
