//! Attributes of the Illuminance Level Sensing cluster.

use zb_core::Cluster;
use zb_core::types::Uint16;

pub use self::types::LevelStatus;
pub use crate::illuminance_measurement::LightSensorType;
use crate::macros::zcl_attributes;

mod types;

zcl_attributes! {
    cluster: Cluster::IlluminanceLevelSensing;

    /// The level status.
    LevelStatus = 0x0000: LevelStatus { R, P },
    /// The light sensor type.
    LightSensorType = 0x0001: LightSensorType { R },
    /// Target illuminance level in lux.
    IlluminanceTargetLevel = 0x0010: Uint16 { R, W },
}
