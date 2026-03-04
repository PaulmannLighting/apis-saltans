//! Readable attributees.

use crate::general::{
    alarms, basic, device_temperature_configuration, groups, identify, level, on_off,
    power_configuration, scenes, time,
};
use crate::measurement_and_sensing::{
    illuminance_level_sensing, illuminance_measurement, occupancy_sensing,
};

/// Readable attributes across all clusters.
// TODO: Consider renaming this to `Attribute` and providing a Box<[T]> of the respective attribute types.
#[non_exhaustive]
pub enum Attribute {
    Alarms(alarms::attributes::Attribute),
    Basic(basic::read::Attribute),
    DeviceTemperatureConfiguration(device_temperature_configuration::attribute::read::Attribute),
    Groups(groups::Attribute),
    Identify(identify::Attribute),
    Level(level::attribute::read::Attribute),
    OnOff(on_off::attribute::read::Attribute),
    PowerConfiguration(power_configuration::read::Attribute),
    Scenes(scenes::Attribute),
    Time(time::attribute::read::Attribute),
    BallastConfiguration, // TODO: Implement ballast configuration attributes enum.
    ColorControl,         // TODO: Implement color control attributes enum.
    IlluminanceLevelSensing(illuminance_level_sensing::attribute::read::Attribute),
    IlluminanceMeasurement(illuminance_measurement::attribute::Attribute),
    OccupancySensing(occupancy_sensing::attribute::Attribute),
}
