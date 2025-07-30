use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

/// Device Enabled Attribute.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromPrimitive)]
#[repr(u8)]
pub enum DeviceEnabled {
    /// Device is disabled.
    Disabled = 0x00,
    /// Device is enabled.
    Enabled = 0x01,
}

impl From<DeviceEnabled> for u8 {
    fn from(value: DeviceEnabled) -> Self {
        value as u8
    }
}

impl TryFrom<u8> for DeviceEnabled {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or(value)
    }
}
