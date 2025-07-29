use le_stream::derive::{FromLeStream, ToLeStream};

use crate::zcl::lighting::color_control::color_mode::ColorMode;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
#[repr(transparent)]
pub struct MaybeColorMode(u8);

impl From<ColorMode> for MaybeColorMode {
    fn from(value: ColorMode) -> Self {
        Self(value as u8)
    }
}

impl TryFrom<MaybeColorMode> for ColorMode {
    type Error = u8;

    fn try_from(value: MaybeColorMode) -> Result<Self, Self::Error> {
        Self::try_from(value.0)
    }
}
