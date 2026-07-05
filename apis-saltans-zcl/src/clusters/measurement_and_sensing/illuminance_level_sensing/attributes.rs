//! Attributes of the Illuminance Level Sensing cluster.

use apis_saltans_core::ClusterId;
use apis_saltans_core::types::Uint16;

pub use self::types::{LevelStatus, LightSensorType};
use crate::macros::zcl_attributes;

mod types;

zcl_attributes! {
    cluster: ClusterId::IlluminanceLevelSensing;

    /// The level status.
    LevelStatus = 0x0000: LevelStatus { R },
    /// The light sensor type.
    LightSensorType = 0x0001: LightSensorType { R },
    /// Target illuminance level in lux.
    IlluminanceTargetLevel = 0x0010: Uint16 { W },
}
