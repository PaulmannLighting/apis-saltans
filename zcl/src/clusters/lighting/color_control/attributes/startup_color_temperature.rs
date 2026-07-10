use zb_core::types::{Type, Uint16};
use zb_core::units::Mireds;

const PREVIOUS: u16 = 0xffff;

/// The startup color temperature to use.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum StartupColorTemperature {
    /// Set the color temperature to the specified value in mireds.
    Value(Mireds),
    /// Use the previous color temperature value.
    Previous,
}

impl From<StartupColorTemperature> for u16 {
    fn from(value: StartupColorTemperature) -> Self {
        match value {
            StartupColorTemperature::Value(mireds) => mireds.into(),
            StartupColorTemperature::Previous => PREVIOUS,
        }
    }
}

impl From<StartupColorTemperature> for Type {
    fn from(value: StartupColorTemperature) -> Self {
        Self::Uint16(Uint16::new(value.into()))
    }
}

impl TryFrom<u16> for StartupColorTemperature {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value == PREVIOUS {
            Ok(Self::Previous)
        } else {
            Mireds::try_from(value).map(Self::Value)
        }
    }
}

impl TryFrom<Uint16> for StartupColorTemperature {
    type Error = Uint16;

    fn try_from(value: Uint16) -> Result<Self, Self::Error> {
        Self::try_from(value.into_inner()).map_err(|_| value)
    }
}

impl TryFrom<Type> for StartupColorTemperature {
    type Error = Type;

    fn try_from(value: Type) -> Result<Self, Self::Error> {
        if let Type::Uint16(value) = value {
            Self::try_from(value).map_err(Type::Uint16)
        } else {
            Err(value)
        }
    }
}
