use le_stream::ToLeStream;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;

/// The generic class of a device.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromPrimitive, ToPrimitive)]
#[repr(u8)]
pub enum GenericDeviceClass {
    /// A lighting device.
    Lighting = 0x00,
}

impl From<GenericDeviceClass> for u8 {
    fn from(value: GenericDeviceClass) -> Self {
        value as Self
    }
}

impl TryFrom<u8> for GenericDeviceClass {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or(value)
    }
}

impl ToLeStream for GenericDeviceClass {
    type Iter = <u8 as ToLeStream>::Iter;

    fn to_le_stream(self) -> Self::Iter {
        u8::from(self).to_le_stream()
    }
}
