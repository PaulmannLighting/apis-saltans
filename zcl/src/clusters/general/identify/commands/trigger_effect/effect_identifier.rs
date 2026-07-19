use num_enum::{IntoPrimitive, TryFromPrimitive};

/// Available effect identifiers.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Clone, Copy, Debug, Eq, Hash, IntoPrimitive, Ord, PartialEq, PartialOrd, TryFromPrimitive,
)]
#[num_enum(error_type(name = u8, constructor = core::convert::identity))]
#[repr(u8)]
pub enum EffectIdentifier {
    /// Turn light off and on again.
    Blink = 0x00,
    /// Repeatedly turn light off and on again with a longer timing.
    Breathe = 0x01,
    /// Turn light green for one second or flash twice.
    Okay = 0x02,
    /// Change color channel or brightness for some time.
    ChannelChange = 0x0b,
    /// Complete the current effect.
    FinishEffect = 0xfe,
    /// Stop the current effect immediately.
    StopEffect = 0xff,
}
