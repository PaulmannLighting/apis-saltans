use alloc::boxed::Box;
use alloc::vec::Vec;

use le_stream::Prefixed;
use le_stream::derive::{FromLeStream, ToLeStream};

/// An octet string, with a capacity of [`u16::MAX`].
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
#[repr(transparent)]
pub struct OctStr16(Prefixed<u16, Box<[u8]>>);

impl AsRef<[u8]> for OctStr16 {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl TryFrom<Box<[u8]>> for OctStr16 {
    type Error = Box<[u8]>;

    fn try_from(value: Box<[u8]>) -> Result<Self, Self::Error> {
        Prefixed::try_from(value).map(Self)
    }
}

impl TryFrom<Vec<u8>> for OctStr16 {
    type Error = Box<[u8]>;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Self::try_from(value.into_boxed_slice())
    }
}
