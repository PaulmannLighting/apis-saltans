use core::fmt::{self, Display, LowerHex, UpperHex};
use core::str::FromStr;

use num_enum::{IntoPrimitive, TryFromPrimitive};
use thiserror::Error;

/// Known Zigbee application device identifiers.
///
/// Devices can be parsed from their exact variant name, decimal identifier, or hexadecimal
/// identifier with a `0x` prefix.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Clone, Copy, Debug, Eq, Hash, IntoPrimitive, Ord, PartialEq, PartialOrd, TryFromPrimitive,
)]
#[num_enum(error_type(name = u16, constructor = core::convert::identity))]
#[repr(u16)]
pub enum Device {
    /// On/off switch.
    OnOffSwitch = 0x0000,
    /// Level-control switch.
    LevelControlSwitch = 0x0001,
    /// On/off output.
    OnOffOutput = 0x0002,
    /// Level-controllable output.
    LevelControllableOutput = 0x0003,
    /// Scene selector.
    SceneSelector = 0x0004,
    /// Configuration tool.
    ConfigurationTool = 0x0005,
    /// Remote control.
    RemoteControl = 0x0006,
    /// Combined interface.
    CombinedInterface = 0x0007,
    /// Range extender.
    RangeExtender = 0x0008,
    /// Mains-powered outlet.
    MainsPowerOutlet = 0x0009,
    /// Door lock.
    DoorLock = 0x000A,
    /// Door-lock controller.
    DoorLockController = 0x000B,
    /// Simple sensor.
    SimpleSensor = 0x000C,
    /// Consumption-awareness device.
    ConsumptionAwareness = 0x000D,

    /// Home gateway.
    HomeGateway = 0x0050,
    /// Smart plug.
    SmartPlug = 0x0051,
    /// White-goods appliance.
    WhiteGoods = 0x0052,
    /// Meter interface.
    MeterInterface = 0x0053,

    /// On/off light.
    OnOffLight = 0x0100,
    /// Dimmable light.
    DimmableLight = 0x0101,
    /// Color-dimmable light.
    ColorDimmableLight = 0x0102,
    /// On/off light switch.
    OnOffLightSwitch = 0x0103,
    /// Dimmer switch.
    DimmerSwitch = 0x0104,
    /// Color-dimmer switch.
    ColorDimmerSwitch = 0x0105,
    /// Light sensor.
    LightSensor = 0x0106,
    /// Occupancy sensor.
    OccupancySensor = 0x0107,

    /// Shade.
    Shade = 0x0200,
    /// Shade controller.
    ShadeController = 0x0201,
    /// Window-covering device.
    WindowCoveringDevice = 0x0202,
    /// Window-covering controller.
    WindowCoveringController = 0x0203,

    /// Heating and cooling unit.
    HeatingCoolingUnit = 0x0300,
    /// Thermostat.
    Thermostat = 0x0301,
    /// Temperature sensor.
    TemperatureSensor = 0x0302,
    /// Pump.
    Pump = 0x0303,
    /// Pump controller.
    PumpController = 0x0304,
    /// Pressure sensor.
    PressureSensor = 0x0305,
    /// Flow sensor.
    FlowSensor = 0x0306,
    /// Mini-split air conditioner.
    MiniSplitAc = 0x0307,

    /// IAS Control and Indicating Equipment.
    IasCie = 0x0400,
    /// IAS ancillary-control equipment.
    IasAncillaryControl = 0x0401,
    /// IAS zone device.
    IasZone = 0x0402,
    /// IAS warning device.
    IasWarningDevice = 0x0403,
}

impl Device {
    /// Returns the device identifier as a `u16`.
    #[must_use]
    pub const fn as_u16(self) -> u16 {
        self as u16
    }
}

impl Display for Device {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} ({:#06X})", self, self.as_u16())
    }
}

impl FromStr for Device {
    type Err = ParseDeviceError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if let Some(device) = device_from_name(value) {
            return Ok(device);
        }

        Self::try_from(parse_device_identifier(value)?).map_err(|_| ParseDeviceError)
    }
}

impl LowerHex for Device {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        LowerHex::fmt(&self.as_u16(), f)
    }
}

impl UpperHex for Device {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        UpperHex::fmt(&self.as_u16(), f)
    }
}

/// Error returned when parsing an unknown or malformed Zigbee device identifier.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
#[error("invalid Zigbee device identifier")]
pub struct ParseDeviceError;

fn device_from_name(value: &str) -> Option<Device> {
    match value {
        "OnOffSwitch" => Some(Device::OnOffSwitch),
        "LevelControlSwitch" => Some(Device::LevelControlSwitch),
        "OnOffOutput" => Some(Device::OnOffOutput),
        "LevelControllableOutput" => Some(Device::LevelControllableOutput),
        "SceneSelector" => Some(Device::SceneSelector),
        "ConfigurationTool" => Some(Device::ConfigurationTool),
        "RemoteControl" => Some(Device::RemoteControl),
        "CombinedInterface" => Some(Device::CombinedInterface),
        "RangeExtender" => Some(Device::RangeExtender),
        "MainsPowerOutlet" => Some(Device::MainsPowerOutlet),
        "DoorLock" => Some(Device::DoorLock),
        "DoorLockController" => Some(Device::DoorLockController),
        "SimpleSensor" => Some(Device::SimpleSensor),
        "ConsumptionAwareness" => Some(Device::ConsumptionAwareness),
        "HomeGateway" => Some(Device::HomeGateway),
        "SmartPlug" => Some(Device::SmartPlug),
        "WhiteGoods" => Some(Device::WhiteGoods),
        "MeterInterface" => Some(Device::MeterInterface),
        "OnOffLight" => Some(Device::OnOffLight),
        "DimmableLight" => Some(Device::DimmableLight),
        "ColorDimmableLight" => Some(Device::ColorDimmableLight),
        "OnOffLightSwitch" => Some(Device::OnOffLightSwitch),
        "DimmerSwitch" => Some(Device::DimmerSwitch),
        "ColorDimmerSwitch" => Some(Device::ColorDimmerSwitch),
        "LightSensor" => Some(Device::LightSensor),
        "OccupancySensor" => Some(Device::OccupancySensor),
        "Shade" => Some(Device::Shade),
        "ShadeController" => Some(Device::ShadeController),
        "WindowCoveringDevice" => Some(Device::WindowCoveringDevice),
        "WindowCoveringController" => Some(Device::WindowCoveringController),
        "HeatingCoolingUnit" => Some(Device::HeatingCoolingUnit),
        "Thermostat" => Some(Device::Thermostat),
        "TemperatureSensor" => Some(Device::TemperatureSensor),
        "Pump" => Some(Device::Pump),
        "PumpController" => Some(Device::PumpController),
        "PressureSensor" => Some(Device::PressureSensor),
        "FlowSensor" => Some(Device::FlowSensor),
        "MiniSplitAc" => Some(Device::MiniSplitAc),
        "IasCie" => Some(Device::IasCie),
        "IasAncillaryControl" => Some(Device::IasAncillaryControl),
        "IasZone" => Some(Device::IasZone),
        "IasWarningDevice" => Some(Device::IasWarningDevice),
        _ => None,
    }
}

fn parse_device_identifier(value: &str) -> Result<u16, ParseDeviceError> {
    value.strip_prefix("0x").map_or_else(
        || value.parse().map_err(|_| ParseDeviceError),
        |value| u16::from_str_radix(value, 16).map_err(|_| ParseDeviceError),
    )
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use alloc::format;
    use alloc::string::ToString;

    use super::{Device, ParseDeviceError};

    const COLOR_DIMMABLE_LIGHT_ID: u16 = 0x0102;
    const COLOR_DIMMABLE_LIGHT_NAME: &str = "ColorDimmableLight";
    const COLOR_DIMMABLE_LIGHT_DISPLAY: &str = "ColorDimmableLight (0x0102)";
    const DOOR_LOCK_LOWER_HEX: &str = "0x000a";
    const DOOR_LOCK_UPPER_HEX: &str = "0x000A";

    #[test]
    fn returns_numeric_identifier() {
        assert_eq!(Device::ColorDimmableLight.as_u16(), COLOR_DIMMABLE_LIGHT_ID);
    }

    #[test]
    fn displays_name_and_numeric_identifier() {
        assert_eq!(
            Device::ColorDimmableLight.to_string(),
            COLOR_DIMMABLE_LIGHT_DISPLAY
        );
    }

    #[test]
    fn formats_lower_hexadecimal_identifier() {
        assert_eq!(format!("{:#06x}", Device::DoorLock), DOOR_LOCK_LOWER_HEX);
    }

    #[test]
    fn formats_upper_hexadecimal_identifier() {
        assert_eq!(format!("{:#06X}", Device::DoorLock), DOOR_LOCK_UPPER_HEX);
    }

    #[test]
    fn parses_name() {
        assert_eq!(
            COLOR_DIMMABLE_LIGHT_NAME.parse(),
            Ok(Device::ColorDimmableLight)
        );
    }

    #[test]
    fn parses_decimal_identifier() {
        assert_eq!(
            COLOR_DIMMABLE_LIGHT_ID.to_string().parse(),
            Ok(Device::ColorDimmableLight)
        );
    }

    #[test]
    fn parses_hexadecimal_identifier() {
        assert_eq!("0x0102".parse(), Ok(Device::ColorDimmableLight));
    }

    #[test]
    fn converts_to_and_from_numeric_identifier() {
        assert_eq!(
            u16::from(Device::ColorDimmableLight),
            COLOR_DIMMABLE_LIGHT_ID
        );
        assert_eq!(
            Device::try_from(COLOR_DIMMABLE_LIGHT_ID),
            Ok(Device::ColorDimmableLight)
        );
    }

    #[test]
    fn rejects_unknown_device() {
        assert_eq!("Unknown".parse::<Device>(), Err(ParseDeviceError));
        assert_eq!("0xFFFF".parse::<Device>(), Err(ParseDeviceError));
        assert_eq!(Device::try_from(u16::MAX), Err(u16::MAX));
    }

    #[test]
    fn rejects_unsupported_representations() {
        assert_eq!(
            COLOR_DIMMABLE_LIGHT_DISPLAY.parse::<Device>(),
            Err(ParseDeviceError)
        );
        assert_eq!("0X0102".parse::<Device>(), Err(ParseDeviceError));
    }
}
