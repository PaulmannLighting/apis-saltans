//! General commands that are not specific to any cluster.

use crate::macros::zcl_command_enum;

pub mod configure_reporting;
pub mod default_response;
pub mod discover_attributes;
pub mod discover_attributes_extended;
pub mod discover_commands_generated;
pub mod discover_commands_received;
pub mod read_attributes;
pub mod read_attributes_structured;
pub mod read_reporting_configuration;
pub mod report_attributes;
pub mod selector;
pub mod write_attributes;
pub mod write_attributes_no_response;
pub mod write_attributes_structured;
pub mod write_attributes_undivided;

pub use self::selector::Selector;

// Available global commands.
zcl_command_enum! {
    Global;
    ReadAttributes(read_attributes::Command),
    ReadAttributesResponse(read_attributes::Response),
    WriteAttributes(write_attributes::Command),
    WriteAttributesUndivided(write_attributes_undivided::Command),
    WriteAttributesResponse(write_attributes::Response),
    WriteAttributesNoResponse(write_attributes_no_response::Command),
    ConfigureReporting(configure_reporting::ConfigureReporting),
    ConfigureReportingResponse(configure_reporting::Response),
    ReadReportingConfiguration(read_reporting_configuration::Command),
    ReadReportingConfigurationResponse(read_reporting_configuration::Response),
    ReportAttributes(report_attributes::Command),
    DefaultResponse(default_response::DefaultResponse),
    DiscoverAttributes(discover_attributes::Command),
    DiscoverAttributesResponse(discover_attributes::Response),
    ReadAttributesStructured(read_attributes_structured::Command),
    WriteAttributesStructured(write_attributes_structured::Command),
    WriteAttributesStructuredResponse(write_attributes_structured::Response),
    DiscoverCommandsReceived(discover_commands_received::Command),
    DiscoverCommandsReceivedResponse(discover_commands_received::Response),
    DiscoverCommandsGenerated(discover_commands_generated::Command),
    DiscoverCommandsGeneratedResponse(discover_commands_generated::Response),
    DiscoverAttributesExtended(discover_attributes_extended::Command),
    DiscoverAttributesExtendedResponse(discover_attributes_extended::Response),
}
