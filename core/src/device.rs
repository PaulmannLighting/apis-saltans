use core::fmt::{self, LowerHex, UpperHex};

use num_enum::{IntoPrimitive, TryFromPrimitive};
use thiserror::Error;

/// Known Zigbee application device identifiers.
///
/// Devices can be parsed from their exact variant name, decimal identifier, or hexadecimal
/// identifier with a `0x` prefix.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    IntoPrimitive,
    Ord,
    PartialEq,
    PartialOrd,
    strum::Display,
    strum::EnumString,
    TryFromPrimitive,
)]
#[cfg_attr(test, derive(strum::EnumIter))]
#[num_enum(error_type(name = u16, constructor = core::convert::identity))]
#[strum(parse_err_ty = ParseDeviceError, parse_err_fn = parse_device_error)]
#[repr(u16)]
pub enum Device {
    /// On/off switch.
    #[strum(
        to_string = "OnOffSwitch (0x0000)",
        serialize = "OnOffSwitch",
        serialize = "0",
        serialize = "0x0000"
    )]
    OnOffSwitch = 0x0000,
    /// Level-control switch.
    #[strum(
        to_string = "LevelControlSwitch (0x0001)",
        serialize = "LevelControlSwitch",
        serialize = "1",
        serialize = "0x0001"
    )]
    LevelControlSwitch = 0x0001,
    /// On/off output.
    #[strum(
        to_string = "OnOffOutput (0x0002)",
        serialize = "OnOffOutput",
        serialize = "2",
        serialize = "0x0002"
    )]
    OnOffOutput = 0x0002,
    /// Level-controllable output.
    #[strum(
        to_string = "LevelControllableOutput (0x0003)",
        serialize = "LevelControllableOutput",
        serialize = "3",
        serialize = "0x0003"
    )]
    LevelControllableOutput = 0x0003,
    /// Scene selector.
    #[strum(
        to_string = "SceneSelector (0x0004)",
        serialize = "SceneSelector",
        serialize = "4",
        serialize = "0x0004"
    )]
    SceneSelector = 0x0004,
    /// Configuration tool.
    #[strum(
        to_string = "ConfigurationTool (0x0005)",
        serialize = "ConfigurationTool",
        serialize = "5",
        serialize = "0x0005"
    )]
    ConfigurationTool = 0x0005,
    /// Remote control.
    #[strum(
        to_string = "RemoteControl (0x0006)",
        serialize = "RemoteControl",
        serialize = "6",
        serialize = "0x0006"
    )]
    RemoteControl = 0x0006,
    /// Combined interface.
    #[strum(
        to_string = "CombinedInterface (0x0007)",
        serialize = "CombinedInterface",
        serialize = "7",
        serialize = "0x0007"
    )]
    CombinedInterface = 0x0007,
    /// Range extender.
    #[strum(
        to_string = "RangeExtender (0x0008)",
        serialize = "RangeExtender",
        serialize = "8",
        serialize = "0x0008"
    )]
    RangeExtender = 0x0008,
    /// Mains-powered outlet.
    #[strum(
        to_string = "MainsPowerOutlet (0x0009)",
        serialize = "MainsPowerOutlet",
        serialize = "9",
        serialize = "0x0009"
    )]
    MainsPowerOutlet = 0x0009,
    /// Door lock.
    #[strum(
        to_string = "DoorLock (0x000A)",
        serialize = "DoorLock",
        serialize = "10",
        serialize = "0x000A",
        serialize = "0x000a"
    )]
    DoorLock = 0x000A,
    /// Door-lock controller.
    #[strum(
        to_string = "DoorLockController (0x000B)",
        serialize = "DoorLockController",
        serialize = "11",
        serialize = "0x000B",
        serialize = "0x000b"
    )]
    DoorLockController = 0x000B,
    /// Simple sensor.
    #[strum(
        to_string = "SimpleSensor (0x000C)",
        serialize = "SimpleSensor",
        serialize = "12",
        serialize = "0x000C",
        serialize = "0x000c"
    )]
    SimpleSensor = 0x000C,
    /// Consumption-awareness device.
    #[strum(
        to_string = "ConsumptionAwareness (0x000D)",
        serialize = "ConsumptionAwareness",
        serialize = "13",
        serialize = "0x000D",
        serialize = "0x000d"
    )]
    ConsumptionAwareness = 0x000D,

    /// Home gateway.
    #[strum(
        to_string = "HomeGateway (0x0050)",
        serialize = "HomeGateway",
        serialize = "80",
        serialize = "0x0050"
    )]
    HomeGateway = 0x0050,
    /// Smart plug.
    #[strum(
        to_string = "SmartPlug (0x0051)",
        serialize = "SmartPlug",
        serialize = "81",
        serialize = "0x0051"
    )]
    SmartPlug = 0x0051,
    /// White-goods appliance.
    #[strum(
        to_string = "WhiteGoods (0x0052)",
        serialize = "WhiteGoods",
        serialize = "82",
        serialize = "0x0052"
    )]
    WhiteGoods = 0x0052,
    /// Meter interface.
    #[strum(
        to_string = "MeterInterface (0x0053)",
        serialize = "MeterInterface",
        serialize = "83",
        serialize = "0x0053"
    )]
    MeterInterface = 0x0053,

    /// On/off light.
    #[strum(
        to_string = "OnOffLight (0x0100)",
        serialize = "OnOffLight",
        serialize = "256",
        serialize = "0x0100"
    )]
    OnOffLight = 0x0100,
    /// Dimmable light.
    #[strum(
        to_string = "DimmableLight (0x0101)",
        serialize = "DimmableLight",
        serialize = "257",
        serialize = "0x0101"
    )]
    DimmableLight = 0x0101,
    /// Color-dimmable light.
    #[strum(
        to_string = "ColorDimmableLight (0x0102)",
        serialize = "ColorDimmableLight",
        serialize = "258",
        serialize = "0x0102"
    )]
    ColorDimmableLight = 0x0102,
    /// On/off light switch.
    #[strum(
        to_string = "OnOffLightSwitch (0x0103)",
        serialize = "OnOffLightSwitch",
        serialize = "259",
        serialize = "0x0103"
    )]
    OnOffLightSwitch = 0x0103,
    /// Dimmer switch.
    #[strum(
        to_string = "DimmerSwitch (0x0104)",
        serialize = "DimmerSwitch",
        serialize = "260",
        serialize = "0x0104"
    )]
    DimmerSwitch = 0x0104,
    /// Color-dimmer switch.
    #[strum(
        to_string = "ColorDimmerSwitch (0x0105)",
        serialize = "ColorDimmerSwitch",
        serialize = "261",
        serialize = "0x0105"
    )]
    ColorDimmerSwitch = 0x0105,
    /// Light sensor.
    #[strum(
        to_string = "LightSensor (0x0106)",
        serialize = "LightSensor",
        serialize = "262",
        serialize = "0x0106"
    )]
    LightSensor = 0x0106,
    /// Occupancy sensor.
    #[strum(
        to_string = "OccupancySensor (0x0107)",
        serialize = "OccupancySensor",
        serialize = "263",
        serialize = "0x0107"
    )]
    OccupancySensor = 0x0107,

    /// Shade.
    #[strum(
        to_string = "Shade (0x0200)",
        serialize = "Shade",
        serialize = "512",
        serialize = "0x0200"
    )]
    Shade = 0x0200,
    /// Shade controller.
    #[strum(
        to_string = "ShadeController (0x0201)",
        serialize = "ShadeController",
        serialize = "513",
        serialize = "0x0201"
    )]
    ShadeController = 0x0201,
    /// Window-covering device.
    #[strum(
        to_string = "WindowCoveringDevice (0x0202)",
        serialize = "WindowCoveringDevice",
        serialize = "514",
        serialize = "0x0202"
    )]
    WindowCoveringDevice = 0x0202,
    /// Window-covering controller.
    #[strum(
        to_string = "WindowCoveringController (0x0203)",
        serialize = "WindowCoveringController",
        serialize = "515",
        serialize = "0x0203"
    )]
    WindowCoveringController = 0x0203,

    /// Heating and cooling unit.
    #[strum(
        to_string = "HeatingCoolingUnit (0x0300)",
        serialize = "HeatingCoolingUnit",
        serialize = "768",
        serialize = "0x0300"
    )]
    HeatingCoolingUnit = 0x0300,
    /// Thermostat.
    #[strum(
        to_string = "Thermostat (0x0301)",
        serialize = "Thermostat",
        serialize = "769",
        serialize = "0x0301"
    )]
    Thermostat = 0x0301,
    /// Temperature sensor.
    #[strum(
        to_string = "TemperatureSensor (0x0302)",
        serialize = "TemperatureSensor",
        serialize = "770",
        serialize = "0x0302"
    )]
    TemperatureSensor = 0x0302,
    /// Pump.
    #[strum(
        to_string = "Pump (0x0303)",
        serialize = "Pump",
        serialize = "771",
        serialize = "0x0303"
    )]
    Pump = 0x0303,
    /// Pump controller.
    #[strum(
        to_string = "PumpController (0x0304)",
        serialize = "PumpController",
        serialize = "772",
        serialize = "0x0304"
    )]
    PumpController = 0x0304,
    /// Pressure sensor.
    #[strum(
        to_string = "PressureSensor (0x0305)",
        serialize = "PressureSensor",
        serialize = "773",
        serialize = "0x0305"
    )]
    PressureSensor = 0x0305,
    /// Flow sensor.
    #[strum(
        to_string = "FlowSensor (0x0306)",
        serialize = "FlowSensor",
        serialize = "774",
        serialize = "0x0306"
    )]
    FlowSensor = 0x0306,
    /// Mini-split air conditioner.
    #[strum(
        to_string = "MiniSplitAc (0x0307)",
        serialize = "MiniSplitAc",
        serialize = "775",
        serialize = "0x0307"
    )]
    MiniSplitAc = 0x0307,

    /// IAS Control and Indicating Equipment.
    #[strum(
        to_string = "IasCie (0x0400)",
        serialize = "IasCie",
        serialize = "1024",
        serialize = "0x0400"
    )]
    IasCie = 0x0400,
    /// IAS ancillary-control equipment.
    #[strum(
        to_string = "IasAncillaryControl (0x0401)",
        serialize = "IasAncillaryControl",
        serialize = "1025",
        serialize = "0x0401"
    )]
    IasAncillaryControl = 0x0401,
    /// IAS zone device.
    #[strum(
        to_string = "IasZone (0x0402)",
        serialize = "IasZone",
        serialize = "1026",
        serialize = "0x0402"
    )]
    IasZone = 0x0402,
    /// IAS warning device.
    #[strum(
        to_string = "IasWarningDevice (0x0403)",
        serialize = "IasWarningDevice",
        serialize = "1027",
        serialize = "0x0403"
    )]
    IasWarningDevice = 0x0403,
}

impl Device {
    /// Returns the device identifier as a `u16`.
    #[must_use]
    pub const fn as_u16(self) -> u16 {
        self as u16
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

const fn parse_device_error(_: &str) -> ParseDeviceError {
    ParseDeviceError
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use alloc::format;
    use alloc::string::ToString;

    use strum::IntoEnumIterator;

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
    fn display_and_parsing_round_trip() {
        for device in Device::iter() {
            assert_eq!(device.to_string().parse(), Ok(device));
        }
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
        assert_eq!("0X0102".parse::<Device>(), Err(ParseDeviceError));
    }
}
