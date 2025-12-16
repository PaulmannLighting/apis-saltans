//! General commands that are not specific to any cluster.

use zigbee::Cluster;
use zigbee_macros::ParseZclFrame;

mod default_response;
pub mod read_attributes;
mod report_attributes;

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
}

impl Cluster for Command {
    const ID: u16 = 0x0000;
}
