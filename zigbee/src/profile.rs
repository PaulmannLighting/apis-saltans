/// Defines the Zigbee Profile Identifiers as per the Zigbee specification.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
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
