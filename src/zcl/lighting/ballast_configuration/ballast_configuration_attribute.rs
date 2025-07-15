use num_derive::FromPrimitive;

/// Ballast Configuration attributes.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, FromPrimitive)]
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
