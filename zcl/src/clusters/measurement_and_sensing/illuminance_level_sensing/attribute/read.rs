//! Readable attributes.

use repr_discriminant::ReprDiscriminant;

use super::level_status::LevelStatus;
use super::light_sensor_type::LightSensorType;

/// Attributes for the illuminance level sensing cluster.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(u16)]
#[derive(ReprDiscriminant)]
pub enum Attribute {
    /// The level status.
    LevelStatus(LevelStatus) = 0x0000,
    /// The light sensor type.
    LightSensorType(LightSensorType) = 0x0001,
}
