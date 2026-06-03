//! General commands that are not specific to any cluster.

use alloc::boxed::Box;

use le_stream::ToLeStream;
use zigbee::Direction;
use zigbee_macros::ParseZclFrame;

use crate::{CommandDispatch, Scope};

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

impl CommandDispatch for Command {
    fn command_id(&self) -> u8 {
        match self {
            Self::ReadAttributes(_) => 0x00,
            Self::ReadAttributesResponse(_) => 0x01,
            Self::ReportAttributes(_) => 0x0A,
            Self::DefaultResponse(_) => 0x0B,
            Self::ConfigureReporting(_) => 0x06,
            Self::ConfigureReportingResponse(_) => 0x07,
        }
    }

    fn scope(&self) -> Scope {
        Scope::Global
    }

    fn direction(&self) -> Direction {
        match self {
            Self::ReadAttributes(_) => Direction::ClientToServer,
            Self::ReadAttributesResponse(_) => Direction::ServerToClient,
            Self::ReportAttributes(_) => Direction::ServerToClient,
            Self::DefaultResponse(_) => Direction::ClientToServer,
            Self::ConfigureReporting(_) => Direction::ClientToServer,
            Self::ConfigureReportingResponse(_) => Direction::ServerToClient,
        }
    }

    fn disable_default_response(&self) -> bool {
        match self {
            Self::DefaultResponse(_) => true,
            Self::ReadAttributes(_)
            | Self::ReadAttributesResponse(_)
            | Self::ReportAttributes(_)
            | Self::ConfigureReporting(_)
            | Self::ConfigureReportingResponse(_) => false,
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
