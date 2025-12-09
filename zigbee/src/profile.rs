use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

/// Defines the Zigbee Profile Identifiers as per the Zigbee specification.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, FromPrimitive)]
#[repr(u16)]
pub enum Profile {
    /// Profile Identifier for Unspecified Profile.
    Unspecified = 0x0000,
    /// Profile Identifier for Home Automation Profile.
    ZigbeeHomeAutomation = 0x0104,
    /// Profile Identifier for Smart Energy Profile.
    SmartEnergy = 0x0109,
    /// Profile Identifier for Light Link Profile.
    Touchlink = 0xC05E,
}

impl TryFrom<u16> for Profile {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::from_u16(value).ok_or(value)
    }
}
