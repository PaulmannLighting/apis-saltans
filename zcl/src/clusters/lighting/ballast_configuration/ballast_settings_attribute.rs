use repr_discriminant::ReprDiscriminant;

use self::level::Level;

mod level;

/// Ballast Settings attributes.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[repr(u16)]
#[derive(ReprDiscriminant)]
pub enum BallastSettingsAttribute {
    /// Minimum light output level.
    MinLevel(Level) = 0x0010,
    /// Maximum light output level.
    MaxLevel(Level) = 0x0011,
    // TODO: Complete implementation.
}
