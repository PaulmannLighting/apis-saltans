use le_stream::{FromLeStream, ToLeStream};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

/// Flag indicating whether the group name is supported by the device.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromPrimitive)]
#[repr(u8)]
pub enum NameSupport {
    /// Group names are not supported by the device.
    Unsupported = 0x00,
    /// Group names are supported by the device.
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

impl FromLeStream for NameSupport {
    fn from_le_stream<T>(bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        u8::from_le_stream(bytes).map(|id| Self::try_from(id).unwrap_or(Self::Unsupported))
    }
}

impl ToLeStream for NameSupport {
    type Iter = <u8 as ToLeStream>::Iter;

    fn to_le_stream(self) -> Self::Iter {
        u8::from(self).to_le_stream()
    }
}
