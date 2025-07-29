use le_stream::derive::{FromLeStream, ToLeStream};

use crate::zcl::lighting::color_control::color_mode::ColorMode;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
pub struct MaybeColorMode(u8);

impl MaybeColorMode {
    /// Returns the inner value.
    ///
    /// # Errors
    ///
    /// If the inner value does not correspond to a valid `ColorMode`, this will return an error.
    pub fn value(self) -> Result<ColorMode, u8> {
        ColorMode::try_from(self.0)
    }
}

impl From<ColorMode> for MaybeColorMode {
    fn from(value: ColorMode) -> Self {
        Self(value as u8)
    }
}
