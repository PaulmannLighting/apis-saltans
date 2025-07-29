use le_stream::derive::{FromLeStream, ToLeStream};

use crate::zcl::lighting::color_control::enhanced_color_mode::EnhancedColorMode;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
#[repr(transparent)]
pub struct MaybeEnhancedColorMode(u8);

impl MaybeEnhancedColorMode {
    /// Returns the inner value.
    ///
    /// # Errors
    ///
    /// If the inner value does not correspond to a valid `EnhancedColorMode`, this will return an error.
    pub fn value(self) -> Result<EnhancedColorMode, u8> {
        EnhancedColorMode::try_from(self.0)
    }
}

impl From<EnhancedColorMode> for MaybeEnhancedColorMode {
    fn from(value: EnhancedColorMode) -> Self {
        Self(value as u8)
    }
}
