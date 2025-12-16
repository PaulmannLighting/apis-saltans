//! General commands that are not specific to any cluster.

use zigbee_macros::ParseZclFrame;

use crate::CommandId;

pub mod configure_reporting;
pub mod default_response;
pub mod read_attributes;
pub mod report_attributes;

/// Available global commands.
#[derive(Clone, Debug, Eq, PartialEq, Hash, ParseZclFrame)]
pub enum Command {
    /// Read Attributes command.
    ReadAttributes(read_attributes::Command),
    /// Read Attributes Response command.
    ReadAttributesResponse(read_attributes::Response),
    /// Report Attributes command.
    ReportAttributes(report_attributes::Command),
    /// Default Response command.
    DefaultResponse(default_response::DefaultResponse),
    /// Configure Reporting command.
    ConfigureReporting(configure_reporting::Command),
    /// Configure Reporting Response command.
    ConfigureReportingResponse(configure_reporting::Response),
}

impl CommandId for Command {
    fn command_id(&self) -> u8 {
        match self {
            Self::ReadAttributes(cmd) => cmd.command_id(),
            Self::ReadAttributesResponse(resp) => resp.command_id(),
            Self::ReportAttributes(cmd) => cmd.command_id(),
            Self::DefaultResponse(cmd) => cmd.command_id(),
            Self::ConfigureReporting(cmd) => cmd.command_id(),
            Self::ConfigureReportingResponse(resp) => resp.command_id(),
        }
    }
}
