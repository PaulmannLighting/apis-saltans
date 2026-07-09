use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use crate::Endpoint;

/// Trait for types that belong to a Zigbee profile.
pub trait Profiled {
    /// The Zigbee profile.
    const PROFILE: Profile;
}

/// Defines the Zigbee Profile Identifiers as per the Zigbee specification.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, FromPrimitive)]
#[repr(u16)]
pub enum Profile {
    /// Profile Identifier for Zigbee Device Profile(ZDP).
    Network = 0x0000,

    /// Profile Identifier for Home Automation Profile.
    ZigbeeHomeAutomation = 0x0104,

    /// Profile Identifier for Building Automation Profile.
    BuildingAutomation = 0x0105,

    /// Profile Identifier for Remote Control Profile.
    RemoteControl = 0x0107,

    /// Profile Identifier for Health Care Profile.
    HealthCare = 0x0108,

    /// Profile Identifier for Smart Energy Profile.
    SmartEnergy = 0x0109,

    /// Profile Identifier for Light Link Profile.
    TouchLink = 0xC05E,
}

impl Profile {
    /// Return the endpoint used for profile-level broadcasts.
    ///
    /// The Zigbee Device Profile uses the data endpoint, while application
    /// profiles use the broadcast endpoint.
    #[must_use]
    pub const fn broadcast_endpoint(self) -> Endpoint {
        if matches!(self, Self::Network) {
            Endpoint::Data
        } else {
            Endpoint::Broadcast
        }
    }
}

impl TryFrom<u16> for Profile {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::from_u16(value).ok_or(value)
    }
}

impl From<Profile> for u16 {
    fn from(profile: Profile) -> Self {
        profile as Self
    }
}
