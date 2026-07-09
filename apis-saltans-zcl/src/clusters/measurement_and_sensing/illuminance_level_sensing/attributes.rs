//! Attributes of the Illuminance Level Sensing cluster.

use apis_saltans_core::Cluster;
use apis_saltans_core::types::Uint16;

pub use self::types::LevelStatus;
use crate::macros::zcl_attributes;
pub use crate::measurement_and_sensing::illuminance_measurement::attributes::LightSensorType;

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
