//! Attributes of the illuminance measurement cluster.

use repr_discriminant::ReprDiscriminant;
use zigbee::types::Uint16;

use super::LightSensorType;

/// Attributes for the Illuminance Measurement cluster.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(u16)]
#[derive(ReprDiscriminant)]
pub enum Attribute {
    /// The measured illuminance value.
    /// The unit of the measured illuminance value is lux.
    /// The non-value of 0xFFFF indicates that the measured illuminance value is invalid.
    MeasuredValue(Uint16) = 0x0000,
    /// The minimum measured illuminance value that can be measured by the device.
    /// The unit of the minimum measured illuminance value is lux.
    /// The non-value of 0xFFFF indicates that the minimum measured illuminance value is invalid.
    MinMeasuredValue(Uint16) = 0x0001,
    /// The maximum measured illuminance value that can be measured by the device.
    /// The unit of the maximum measured illuminance value is lux.
    /// The non-value of 0xFFFF indicates that the maximum measured illuminance value is invalid.
    MaxMeasuredValue(Uint16) = 0x0002,
    /// The tolerance of the measured illuminance value.
    Tolerance(Uint16) = 0x0003,
    /// The type of light sensor used to measure the illuminance.
    LightSensorType(LightSensorType) = 0x0004,
}
