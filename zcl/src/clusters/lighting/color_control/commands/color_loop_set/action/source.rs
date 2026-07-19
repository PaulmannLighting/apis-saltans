use num_enum::{IntoPrimitive, TryFromPrimitive};

/// Activation source for the color loop.
#[derive(
    Clone, Copy, Debug, Eq, Hash, IntoPrimitive, Ord, PartialEq, PartialOrd, TryFromPrimitive,
)]
#[repr(u8)]
pub enum Source {
    /// Activate the color loop from the value in the `ColorLoopStartEnhancedHue` field.
    ColorLoopStartEnhancedHue = 0x01,
    /// Activate the color loop from the value of the `EnhancedCurrentHue` attribute.
    EnhancedCurrentHue = 0x02,
}
