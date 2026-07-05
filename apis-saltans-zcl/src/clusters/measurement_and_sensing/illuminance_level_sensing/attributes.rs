//! Attributes of the Illuminance Level Sensing cluster.

use apis_saltans_core::ClusterId;
use apis_saltans_core::types::{Type, Uint16};

use crate::macros::zcl_attributes;

zcl_attributes! {
    cluster: ClusterId::IlluminanceLevelSensing;

    /// The level status.
    LevelStatus = 0x0000: Type { R },
    /// The light sensor type.
    LightSensorType = 0x0001: Type { R },
    /// Target illuminance level in lux.
    IlluminanceTargetLevel = 0x0010: Uint16 { W },
}
