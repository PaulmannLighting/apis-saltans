//! General ZCL clusters.

pub mod basic;
pub mod device_temperature_configuration;
pub mod groups;
pub mod identify;
pub mod on_off;
pub mod power_configuration;
pub mod scenes;

/// Commands for the General clusters.
#[expect(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum Command {
    /// Basic cluster commands.
    Basic(basic::Command),
    /// Device Temperature Configuration cluster commands.
    DeviceTemperatureConfiguration(device_temperature_configuration::Command),
    /// Groups cluster commands.
    Groups(groups::Command),
    /// Identify cluster commands.
    Identify(identify::Command),
    /// On/Off cluster commands.
    OnOff(on_off::Command),
    /// Power Configuration cluster commands.
    PowerConfiguration(power_configuration::Command),
    /// Scenes cluster commands.
    Scenes(scenes::Command),
}

/// Responses for the General clusters.
#[expect(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum Response {
    /// Basic cluster responses.
    Basic(basic::Response),
    /// Device Temperature Configuration cluster responses.
    DeviceTemperatureConfiguration(device_temperature_configuration::Response),
    /// Groups cluster responses.
    Groups(groups::Response),
    /// Identify cluster responses.
    Identify(identify::Response),
    /// On/Off cluster responses.
    OnOff(on_off::Response),
    /// Power Configuration cluster responses.
    PowerConfiguration(power_configuration::Response),
    /// Scenes cluster responses.
    Scenes(scenes::Response),
}
