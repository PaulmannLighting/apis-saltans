use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use zb_core::types::{Type as ZclType, Uint16};

/// Zone types.
///
/// TODO: Add option for manufacturer-specific types.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, FromPrimitive)]
#[repr(u16)]
pub enum Type {
    /// Standard CIE
    StandardCie = 0x0000,

    /// Motion sensor
    MotionSensor = 0x000d,

    /// Contact switch
    ContactSwitch = 0x0015,

    /// Door/window handle
    DoorWindowHandle = 0x0016,

    /// Fire sensor
    FireSensor = 0x0028,

    /// Water sensor
    WaterSensor = 0x002a,

    /// Carbon monoxide sensor
    CarbonMonoxideSensor = 0x002b,

    /// Personal emergency device
    PersonalEmergencyDevice = 0x002c,

    /// Vibration/movement sensor
    VibrationMovementSensor = 0x002d,

    /// Remote control
    RemoteControl = 0x010f,

    /// Key fob
    KeyFob = 0x0115,

    /// Keypad
    Keypad = 0x021d,

    /// Standard warning device
    StandardWarningDevice = 0x0225,

    /// Glass break sensor
    GlassBreakSensor = 0x0226,

    /// Security repeater
    SecurityRepeater = 0x0229,

    /// Invalid zone type
    Invalid = 0xffff,
}

impl From<Type> for u16 {
    fn from(value: Type) -> Self {
        value as Self
    }
}

impl From<Type> for ZclType {
    fn from(value: Type) -> Self {
        Self::Enum16(Uint16::new(value.into()))
    }
}

impl TryFrom<u16> for Type {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::from_u16(value).ok_or(value)
    }
}

impl TryFrom<Uint16> for Type {
    type Error = Uint16;

    fn try_from(value: Uint16) -> Result<Self, Self::Error> {
        Self::try_from(value.into_inner()).map_err(|_| value)
    }
}

impl TryFrom<ZclType> for Type {
    type Error = ZclType;

    fn try_from(value: ZclType) -> Result<Self, Self::Error> {
        if let ZclType::Enum16(value) = value {
            Self::try_from(value).map_err(ZclType::Enum16)
        } else {
            Err(value)
        }
    }
}
