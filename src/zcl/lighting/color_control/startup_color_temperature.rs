use crate::units::Mireds;

const PREVIOUS: u16 = 0xffff;

/// The startup color temperature to use.
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
