use le_stream::derive::{FromLeStream, ToLeStream};

use crate::zcl::lighting::color_control::enhanced_color_mode::EnhancedColorMode;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
#[repr(transparent)]
pub struct MaybeEnhancedColorMode(u8);

impl From<EnhancedColorMode> for MaybeEnhancedColorMode {
    fn from(value: EnhancedColorMode) -> Self {
        Self(value as u8)
    }
}

impl TryFrom<MaybeEnhancedColorMode> for EnhancedColorMode {
    type Error = u8;

    fn try_from(value: MaybeEnhancedColorMode) -> Result<Self, Self::Error> {
        Self::try_from(value.0)
    }
}
