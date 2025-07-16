use le_stream::{FromLeStream, ToLeStream};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

/// Device Enabled Attribute.

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromPrimitive)]
#[repr(u8)]
pub enum DeviceEnabled {
    /// Device is disabled.
    Disabled = 0x00,
    /// Device is enabled.
    Enabled = 0x01,
}

impl FromLeStream for DeviceEnabled {
    fn from_le_stream<T>(bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        u8::from_le_stream(bytes).and_then(Self::from_u8)
    }
}

impl ToLeStream for DeviceEnabled {
    type Iter = <u8 as ToLeStream>::Iter;

    fn to_le_stream(self) -> Self::Iter {
        (self as u8).to_le_stream()
    }
}
