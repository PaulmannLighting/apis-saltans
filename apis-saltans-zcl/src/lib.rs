//! Zigbee Cluster Library (ZCL) frame, command, cluster, and attribute models.
//!
//! The crate provides typed ZCL frame parsing and serialization, global commands, selected
//! cluster-specific commands, and generated access-specific attribute enums.
//!
//! Runtime command dispatch currently covers global commands plus the Basic, Groups, Identify,
//! On/Off, Level Control, Alarms, Scenes, Color Control, and IAS Zone clusters. Attribute modules
//! are broader and currently cover implemented General, Lighting, Measurement and Sensing, and IAS
//! clusters. Use [`Reportable::parse`] to construct a typed reportable attribute from a cluster ID,
//! attribute ID, and raw [`apis_saltans_core::types::Type`].

pub use self::attributes::{InvalidType, ParseAttributeError, Readable, Reportable, Writable};
pub use self::clusters::{Cluster, general, global, ias, lighting, measurement_and_sensing};
pub use self::command::{Command, CommandDispatch};
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
