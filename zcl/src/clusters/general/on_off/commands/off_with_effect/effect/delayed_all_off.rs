use num_enum::{IntoPrimitive, TryFromPrimitive};

/// Delayed all off effect variants.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, IntoPrimitive, PartialEq, TryFromPrimitive)]
#[num_enum(error_type(name = u8, constructor = core::convert::identity))]
#[repr(u8)]
pub enum DelayedAllOff {
    /// Fade to off in 0.8 seconds.
    #[default]
    FadeToOff = 0x00,
    /// No fade.
    NoFade = 0x01,
    /// 50%% dim down in 0.8 seconds, then fade to off in 12 seconds.
    DimDown = 0x02,
}
