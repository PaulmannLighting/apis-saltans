//! Reporting configuration command for the Global cluster.

use std::boxed::Box;

use apis_saltans_core::Direction;

pub use self::attribute_reporting_configuration::AttributeReportingConfiguration;
pub use self::attribute_status::AttributeStatus;
use crate::macros::zcl_command;

mod attribute_reporting_configuration;
mod attribute_status;

zcl_command! {
    /// Command to configure reporting for attributes.
    Command {
        Global;
        command_id: 0x06;
        direction: Direction::ClientToServer;
        => crate::global::ConfigureReporting(box);
        fields {
            attributes: Box<[AttributeReportingConfiguration]>,
        }

        getters {
            /// Returns the attributes.
            #[must_use]
            pub fn attributes(&self) -> &[AttributeReportingConfiguration] {
                &self.attributes
            }
        }
    }
}

zcl_command! {
    /// Status of an attribute reporting configuration.
    Response {
        Global;
        command_id: 0x07;
        direction: Direction::ServerToClient;
        => crate::global::ConfigureReportingResponse(box);
        fields {
            status: Box<[AttributeStatus]>,
        }

        getters {
            /// Returns the status.
            #[must_use]
            pub fn status(&self) -> &[AttributeStatus] {
                &self.status
            }
        }
    }
}
