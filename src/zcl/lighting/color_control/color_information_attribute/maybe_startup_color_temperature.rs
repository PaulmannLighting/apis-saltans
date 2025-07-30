use le_stream::derive::{FromLeStream, ToLeStream};

use crate::zcl::lighting::color_control::startup_color_temperature::StartupColorTemperature;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
#[repr(transparent)]
pub struct MaybeStartupColorTemperature(u16);

impl TryFrom<MaybeStartupColorTemperature> for StartupColorTemperature {
    type Error = u16;

    fn try_from(value: MaybeStartupColorTemperature) -> Result<Self, Self::Error> {
        Self::try_from(value.0)
    }
}

impl From<StartupColorTemperature> for MaybeStartupColorTemperature {
    fn from(value: StartupColorTemperature) -> Self {
        Self(value.into())
    }
}
