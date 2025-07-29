use le_stream::derive::{FromLeStream, ToLeStream};

use crate::zcl::lighting::color_control::color_loop_direction::ColorLoopDirection;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
#[repr(transparent)]
pub struct MaybeColorLoopDirection(u8);

impl From<ColorLoopDirection> for MaybeColorLoopDirection {
    fn from(value: ColorLoopDirection) -> Self {
        Self(value as u8)
    }
}

impl TryFrom<MaybeColorLoopDirection> for ColorLoopDirection {
    type Error = u8;

    fn try_from(value: MaybeColorLoopDirection) -> Result<Self, Self::Error> {
        Self::try_from(value.0)
    }
}
