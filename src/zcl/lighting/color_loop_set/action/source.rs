use num_derive::FromPrimitive;

/// Activation source for the color loop.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromPrimitive)]
#[repr(u8)]
pub enum Source {
    /// Activate the color loop from the value in the `ColorLoopStartEnhancedHue` field.
    ColorLoopStartEnhancedHue = 0x01,
    /// Activate the color loop from the value of the `EnhancedCurrentHue` attribute.
    EnhancedCurrentHue = 0x02,
}
