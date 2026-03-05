//! Readable attributees.

use alloc::boxed::Box;

use crate::general::{
    alarms, basic, device_temperature_configuration, groups, identify, level, on_off,
    power_configuration, scenes, time,
};
use crate::measurement_and_sensing::{
    illuminance_level_sensing, illuminance_measurement, occupancy_sensing,
};

/// Readable attributes across all clusters.
#[non_exhaustive]
pub enum Attribute {
    Alarms(Box<alarms::readable::Attribute>),
    Basic(Box<basic::readable::Attribute>),
    DeviceTemperatureConfiguration(Box<device_temperature_configuration::readable::Attribute>),
    Groups(Box<groups::readable::Attribute>),
    Identify(Box<identify::readable::Attribute>),
    Level(Box<level::readable::Attribute>),
    OnOff(Box<on_off::readable::Attribute>),
    PowerConfiguration(Box<power_configuration::readable::Attribute>),
    Scenes(Box<scenes::readable::Attribute>),
    Time(Box<time::attribute::read::Attribute>),
    BallastConfiguration, // TODO: Implement ballast configuration attributes enum.
    ColorControl,         // TODO: Implement color control attributes enum.
    IlluminanceLevelSensing(Box<illuminance_level_sensing::attribute::read::Attribute>),
    IlluminanceMeasurement(Box<illuminance_measurement::attribute::Attribute>),
    OccupancySensing(Box<occupancy_sensing::attribute::Attribute>),
}
