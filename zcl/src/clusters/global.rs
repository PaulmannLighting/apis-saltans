//! General commands that are not specific to any cluster.

use alloc::boxed::Box;

use le_stream::ToLeStream;
use zigbee::Direction;
use zigbee_macros::ParseZclFrame;

use crate::{Cluster, CommandDispatch, Scope};

pub mod configure_reporting;
pub mod default_response;
pub mod read_attributes;
pub mod report_attributes;
pub mod write_attributes;
pub mod write_attributes_no_response;
pub mod write_attributes_undivided;

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

impl From<Command> for Cluster {
    fn from(command: Command) -> Self {
        Self::Global(command)
    }
}

impl From<read_attributes::Command> for Command {
    fn from(command: read_attributes::Command) -> Self {
        Self::ReadAttributes(command)
    }
}

impl From<read_attributes::Response> for Command {
    fn from(response: read_attributes::Response) -> Self {
        Self::ReadAttributesResponse(response)
    }
}

impl From<report_attributes::Command> for Command {
    fn from(command: report_attributes::Command) -> Self {
        Self::ReportAttributes(command)
    }
}

impl From<default_response::DefaultResponse> for Command {
    fn from(response: default_response::DefaultResponse) -> Self {
        Self::DefaultResponse(response)
    }
}

impl From<configure_reporting::Command> for Command {
    fn from(command: configure_reporting::Command) -> Self {
        Self::ConfigureReporting(command)
    }
}

impl From<configure_reporting::Response> for Command {
    fn from(response: configure_reporting::Response) -> Self {
        Self::ConfigureReportingResponse(response)
    }
}

impl CommandDispatch for Command {
    fn command_id(&self) -> u8 {
        match self {
            Self::ReadAttributes(cmd) => cmd.command_id(),
            Self::ReadAttributesResponse(cmd) => cmd.command_id(),
            Self::ReportAttributes(cmd) => cmd.command_id(),
            Self::DefaultResponse(cmd) => cmd.command_id(),
            Self::ConfigureReporting(cmd) => cmd.command_id(),
            Self::ConfigureReportingResponse(cmd) => cmd.command_id(),
        }
    }

    fn scope(&self) -> Scope {
        Scope::Global
    }

    fn direction(&self) -> Direction {
        match self {
            Self::ReadAttributes(cmd) => cmd.direction(),
            Self::ReadAttributesResponse(cmd) => cmd.direction(),
            Self::ReportAttributes(cmd) => cmd.direction(),
            Self::DefaultResponse(cmd) => cmd.direction(),
            Self::ConfigureReporting(cmd) => cmd.direction(),
            Self::ConfigureReportingResponse(cmd) => cmd.direction(),
        }
    }

    fn disable_default_response(&self) -> bool {
        match self {
            Self::DefaultResponse(cmd) => cmd.disable_default_response(),
            Self::ReadAttributes(cmd) => cmd.disable_default_response(),
            Self::ReadAttributesResponse(cmd) => cmd.disable_default_response(),
            Self::ReportAttributes(cmd) => cmd.disable_default_response(),
            Self::ConfigureReporting(cmd) => cmd.disable_default_response(),
            Self::ConfigureReportingResponse(cmd) => cmd.disable_default_response(),
        }
    }
}

impl ToLeStream for Command {
    type Iter = Iter;

    fn to_le_stream(self) -> Self::Iter {
        match self {
            Self::ReadAttributes(cmd) => Iter::ReadAttributes(cmd.to_le_stream()),
            Self::ReadAttributesResponse(cmd) => Iter::ReadAttributesResponse(cmd.to_le_stream()),
            Self::ReportAttributes(cmd) => Iter::ReportAttributes(cmd.to_le_stream()),
            Self::DefaultResponse(cmd) => Iter::DefaultResponse(cmd.to_le_stream()),
            Self::ConfigureReporting(cmd) => Iter::ConfigureReporting(cmd.to_le_stream().into()),
            Self::ConfigureReportingResponse(cmd) => {
                Iter::ConfigureReportingResponse(cmd.to_le_stream())
            }
        }
    }
}

#[expect(missing_docs)]
#[derive(Debug)]
pub enum Iter {
    ReadAttributes(<read_attributes::Command as ToLeStream>::Iter),
    ReadAttributesResponse(<read_attributes::Response as ToLeStream>::Iter),
    ReportAttributes(<report_attributes::Command as ToLeStream>::Iter),
    DefaultResponse(<default_response::DefaultResponse as ToLeStream>::Iter),
    ConfigureReporting(Box<<configure_reporting::Command as ToLeStream>::Iter>),
    ConfigureReportingResponse(<configure_reporting::Response as ToLeStream>::Iter),
}

impl Iterator for Iter {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Iter::ReadAttributes(iter) => iter.next(),
            Iter::ReadAttributesResponse(iter) => iter.next(),
            Iter::ReportAttributes(iter) => iter.next(),
            Iter::DefaultResponse(iter) => iter.next(),
            Iter::ConfigureReporting(iter) => iter.next(),
            Iter::ConfigureReportingResponse(iter) => iter.next(),
        }
    }
}
