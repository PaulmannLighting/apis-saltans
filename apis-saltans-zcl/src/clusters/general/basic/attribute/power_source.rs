use le_stream::ToLeStream;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use apis_saltans_core::types::{Type, Uint8};

/// Device power source attribute.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromPrimitive)]
#[repr(u8)]
pub enum PowerSource {
    /// The power source is unknown.
    Unknown = 0x00,
    /// The power source is mains single phase.
    MainsSinglePhase = 0x01,
    /// The power source is mains 3-phase.
    MainsThreePhase = 0x02,
    /// The power source is a battery.
    Battery = 0x03,
    /// The power source is a DC source.
    DcSource = 0x04,
    /// The power source is an emergency mains supply that is constantly powered.
    EmergencyMainsConstantlyPowered = 0x05,
    /// The power source is an emergency mains supply powered through a transfer switch.
    EmergencyMainsAndTransferSwitch = 0x06,
}

impl From<PowerSource> for u8 {
    fn from(value: PowerSource) -> Self {
        value as Self
    }
}

impl From<PowerSource> for Uint8 {
    fn from(value: PowerSource) -> Self {
        Self::new(value.into())
    }
}

impl From<PowerSource> for Type {
    fn from(value: PowerSource) -> Self {
        Self::Enum8(value.into())
    }
}

impl TryFrom<u8> for PowerSource {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or(value)
    }
}

impl TryFrom<Uint8> for PowerSource {
    type Error = Uint8;

    fn try_from(value: Uint8) -> Result<Self, Self::Error> {
        value.as_u8().try_into().map_err(|_| value)
    }
}

impl TryFrom<Type> for PowerSource {
    type Error = Type;

    fn try_from(typ: Type) -> Result<Self, Self::Error> {
        if let Type::Enum8(value) = typ {
            value.try_into().map_err(|_| typ)
        } else {
            Err(typ)
        }
    }
}

impl ToLeStream for PowerSource {
    type Iter = <u8 as ToLeStream>::Iter;

    fn to_le_stream(self) -> Self::Iter {
        u8::from(self).to_le_stream()
    }
}
