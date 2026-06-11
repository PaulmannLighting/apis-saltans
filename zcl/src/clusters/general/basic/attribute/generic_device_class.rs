use le_stream::ToLeStream;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;
use zigbee::types::{Type, Uint8};

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

impl From<GenericDeviceClass> for Uint8 {
    fn from(value: GenericDeviceClass) -> Self {
        Self::new(value.into())
    }
}

impl From<GenericDeviceClass> for Type {
    fn from(value: GenericDeviceClass) -> Self {
        Self::Enum8(value.into())
    }
}

impl TryFrom<u8> for GenericDeviceClass {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or(value)
    }
}

impl TryFrom<Uint8> for GenericDeviceClass {
    type Error = Uint8;

    fn try_from(value: Uint8) -> Result<Self, Self::Error> {
        Self::try_from(value.as_u8()).map_err(|_| value)
    }
}

impl TryFrom<Type> for GenericDeviceClass {
    type Error = Type;

    fn try_from(typ: Type) -> Result<Self, Self::Error> {
        if let Type::Enum8(value) = typ {
            Self::try_from(value).map_err(Type::Enum8)
        } else {
            Err(typ)
        }
    }
}

impl ToLeStream for GenericDeviceClass {
    type Iter = <u8 as ToLeStream>::Iter;

    fn to_le_stream(self) -> Self::Iter {
        u8::from(self).to_le_stream()
    }
}
