use le_stream::derive::{FromLeStream, ToLeStream};

use crate::zcl::lighting::color_control::color_loop_direction::ColorLoopDirection;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
#[repr(transparent)]
pub struct MaybeColorLoopDirection(u8);

impl MaybeColorLoopDirection {
    /// Returns the inner value.
    ///
    /// # Errors
    ///
    /// If the inner value does not correspond to a valid `ColorLoopDirection`, this will return an error.
    pub fn value(self) -> Result<ColorLoopDirection, u8> {
        ColorLoopDirection::try_from(self.0)
    }
}

impl From<ColorLoopDirection> for MaybeColorLoopDirection {
    fn from(value: ColorLoopDirection) -> Self {
        Self(value as u8)
    }
}
