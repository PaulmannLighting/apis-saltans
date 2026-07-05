//! Attributes of the Illuminance Measurement cluster.

use apis_saltans_core::ClusterId;
use apis_saltans_core::types::Uint16;

pub use self::types::{LightSensorType, Lux, ManufacturerSpecific, MeasuredValue};
use crate::macros::zcl_attributes;

mod types;

zcl_attributes! {
    cluster: ClusterId::IlluminanceMeasurement;

    /// The measured illuminance value.
    MeasuredValue = 0x0000: MeasuredValue { R, P },
    /// The minimum measured illuminance value that can be measured by the device.
    MinMeasuredValue = 0x0001: Uint16 { R },
    /// The maximum measured illuminance value that can be measured by the device.
    MaxMeasuredValue = 0x0002: Uint16 { R },
    /// The tolerance of the measured illuminance value.
    Tolerance = 0x0003: Uint16 { R },
    /// The type of light sensor used to measure the illuminance.
    LightSensorType = 0x0004: LightSensorType { R },
}
