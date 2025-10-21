use le_stream::ToLeStream;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;

/// The generic type of device.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromPrimitive, ToPrimitive)]
#[repr(u8)]
pub enum GenericDeviceType {
    /// Incandescent light bulb.
    Incandescent = 0x00,
    /// Halogen spotlight light bulb.
    SpotlightHalogen = 0x01,
    /// Halogen light bulb.
    HalogenBulb = 0x02,
    /// Compact fluorescent light bulb.
    Cfl = 0x03,
    /// Linear fluorescent light bulb.
    LinearFluorescent = 0x04,
    /// LED light bulb.
    LedBulb = 0x05,
    /// LED spotlight light bulb.
    SpotlightLed = 0x06,
    /// LED strip light.
    LedStrip = 0x07,
    /// LED tube light.
    LedTube = 0x08,
    /// Generic indoor luminaire.
    GenericIndoorLuminaire = 0x09,
    /// Generic outdoor luminaire.
    GenericOutdoorLuminaire = 0x0a,
    /// Pendant luminaire.
    PendantLuminaire = 0x0b,
    /// Floor standing luminaire.
    FloorStandingLuminaire = 0x0c,
    /// Generic controller device.
    GenericController = 0xe0,
    /// Table luminaire.
    TableLuminaire = 0x0d,
    /// Wall mounted switch.
    WallSwitch = 0xe1,
    /// Portable remote controller.
    PortableRemoteController = 0xe2,
    /// Motion sensor.
    MotionSensor = 0xe3,
    /// Generic actuator device.
    GenericActuator = 0xf0,
    /// Wall socket.
    WallSocket = 0xf1,
    /// Gateway or bridge device.
    GatewayOrBridge = 0xf2,
    /// Plug-in unit.
    PlugInUnit = 0xf3,
    /// Retrofit actuator.
    RetrofitActuator = 0xf4,
    /// Unspecified device type.
    Unspecified = 0xff,
}

impl From<GenericDeviceType> for u8 {
    fn from(value: GenericDeviceType) -> Self {
        value as Self
    }
}

impl TryFrom<u8> for GenericDeviceType {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or(value)
    }
}

impl ToLeStream for GenericDeviceType {
    type Iter = <u8 as ToLeStream>::Iter;

    fn to_le_stream(self) -> Self::Iter {
        u8::from(self).to_le_stream()
    }
}
