use le_stream::ToLeStream;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use zigbee::types::{Bool, Type};

/// Device Enabled Attribute.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromPrimitive)]
#[repr(u8)]
pub enum DeviceEnabled {
    /// Device is disabled.
    Disabled = 0x00,

    /// Device is enabled.
    #[default]
    Enabled = 0x01,
}

impl From<DeviceEnabled> for u8 {
    fn from(value: DeviceEnabled) -> Self {
        value as Self
    }
}

impl From<bool> for DeviceEnabled {
    fn from(value: bool) -> Self {
        if value { Self::Enabled } else { Self::Disabled }
    }
}

impl From<DeviceEnabled> for bool {
    fn from(value: DeviceEnabled) -> Self {
        match value {
            DeviceEnabled::Disabled => false,
            DeviceEnabled::Enabled => true,
        }
    }
}

impl From<DeviceEnabled> for Bool {
    fn from(value: DeviceEnabled) -> Self {
        match value {
            DeviceEnabled::Disabled => Bool::FALSE,
            DeviceEnabled::Enabled => Bool::TRUE,
        }
    }
}

impl From<DeviceEnabled> for Type {
    fn from(value: DeviceEnabled) -> Self {
        Self::Boolean(value.into())
    }
}

impl TryFrom<Bool> for DeviceEnabled {
    type Error = Bool;

    fn try_from(value: Bool) -> Result<Self, Self::Error> {
        match value {
            Bool::FALSE => Ok(Self::Disabled),
            Bool::TRUE => Ok(Self::Enabled),
            _ => Err(value),
        }
    }
}

impl TryFrom<u8> for DeviceEnabled {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or(value)
    }
}

impl ToLeStream for DeviceEnabled {
    type Iter = <u8 as ToLeStream>::Iter;

    fn to_le_stream(self) -> Self::Iter {
        u8::from(self).to_le_stream()
    }
}
