use le_stream::FromLeStreamTagged;
use zigbee::Parsable;

use self::level::Level;

mod level;

/// Ballast Settings attributes.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[repr(u16)]
#[derive(FromLeStreamTagged)]
pub enum BallastSettingsAttribute {
    /// Minimum light output level.
    MinLevel(Parsable<u8, Level>) = 0x0010,
    /// Maximum light output level.
    MaxLevel(Parsable<u8, Level>) = 0x0011,
    // TODO: Complete implementation.
}
