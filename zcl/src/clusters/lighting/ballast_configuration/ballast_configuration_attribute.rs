use num_enum::{IntoPrimitive, TryFromPrimitive};

/// Ballast Configuration attribute.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, IntoPrimitive, PartialEq, TryFromPrimitive)]
#[num_enum(error_type(name = u16, constructor = core::convert::identity))]
#[repr(u16)]
pub enum BallastConfigurationAttribute {
    /// Ballast information.
    BallastInformation = 0x0000,
    /// Ballast settings.
    BallastSettings = 0x0001,
    /// Lamp information.
    LampInformation = 0x0002,
    /// Lamp settings.
    LampSettings = 0x0003,
}
