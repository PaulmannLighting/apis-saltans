//! Zigbee Cluster Library (ZCL) frame, command, cluster, and attribute models.
//!
//! The crate provides typed ZCL frame parsing and serialization, global commands, selected
//! cluster-specific commands, and generated access-specific attribute enums.
//!
//! Runtime command dispatch currently covers global commands plus the Basic, Groups, Identify,
//! On/Off, Level Control, Alarms, Scenes, OTA Upgrade, Color Control, and IAS Zone clusters.
//! Attribute modules are broader and currently cover implemented General, Lighting, Measurement
//! and Sensing, and IAS clusters. Use [`AttributeReport::parse`] to construct a typed reportable
//! attribute from a cluster ID, attribute ID, and raw [`zb_core::types::Type`].
//!
//! Set `ZCL_DISABLE_DEFAULT_RESPONSE=true` in the build environment to make commands that do not
//! specify their own default-response behavior set the disable-default-response bit in outgoing
//! frame control fields.

pub use self::attributes::{
    Analog, AttributeReport, Discrete, InvalidType, ParseAttributeError, Readable, Reportable,
    Writable,
};
pub use self::clusters::general::{
    alarms, basic, device_temperature_configuration, groups, identify, level, on_off, ota_upgrade,
    power_configuration, scenes, time,
};
pub use self::clusters::lighting::{ballast_configuration, color_control};
pub use self::clusters::measurement_and_sensing::{
    illuminance_level_sensing, illuminance_measurement, occupancy_sensing,
};
pub use self::clusters::{Cluster, global, ias};
pub use self::command::{Command, Directed, ParseDirection, Scoped};
pub use self::frame::{Control, Direction, Frame, Header, ParseFrameError, Scope};
pub use self::options::Options;
pub use self::status::Status;

mod attributes;
mod clusters;
mod command;
mod frame;
mod macros;
mod options;
mod status;
