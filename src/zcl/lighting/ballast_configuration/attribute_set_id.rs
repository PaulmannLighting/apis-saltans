/// Attribute set identifiers for the Ballast Configuration cluster.
pub enum AttributeSetId {
    /// Ballast information.
    BallastInformation = 0x0000,
    /// Ballast settings.
    BallastSettings = 0x0001,
    /// Lamp information.
    LampInformation = 0x0002,
    /// Lamp settings.
    LampSettings = 0x0003,
}
