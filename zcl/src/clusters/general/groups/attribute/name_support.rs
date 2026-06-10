use le_stream::ToLeStream;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

/// Flag indicating whether the group name is supported by the device.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromPrimitive)]
#[repr(u8)]
pub enum NameSupport {
    /// The device does not support group names.
    Unsupported = 0x00,
    /// The device supports group names.
    Supported = 0x01,
}

impl From<NameSupport> for u8 {
    fn from(name_support: NameSupport) -> Self {
        name_support as Self
    }
}

impl TryFrom<u8> for NameSupport {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or(value)
    }
}

impl ToLeStream for NameSupport {
    type Iter = <u8 as ToLeStream>::Iter;

    fn to_le_stream(self) -> Self::Iter {
        u8::from(self).to_le_stream()
    }
}
