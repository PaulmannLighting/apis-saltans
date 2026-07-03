//! General commands that are not specific to any cluster.

use crate::macros::zcl_command_enum;

pub mod configure_reporting;
pub mod default_response;
pub mod read_attributes;
pub mod report_attributes;
pub mod write_attributes;
pub mod write_attributes_no_response;
pub mod write_attributes_undivided;

// Available global commands.
zcl_command_enum! {
    Global;
    ReadAttributes(read_attributes::Command),
    ReadAttributesResponse(read_attributes::Response),
    WriteAttributes(write_attributes::Command),
    WriteAttributesUndivided(write_attributes_undivided::Command),
    WriteAttributesResponse(write_attributes::Response),
    WriteAttributesNoResponse(write_attributes_no_response::Command),
    ReportAttributes(report_attributes::Command),
    DefaultResponse(default_response::DefaultResponse),
    ConfigureReporting(configure_reporting::Command),
    ConfigureReportingResponse(configure_reporting::Response),
}
