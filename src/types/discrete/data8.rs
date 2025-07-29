use le_stream::derive::{FromLeStream, ToLeStream};

/// The `8-bit data` type, short `data8`.
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
#[repr(transparent)]
pub struct Data8([u8; 1]);

impl AsRef<[u8]> for Data8 {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsMut<[u8]> for Data8 {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl From<[u8; 1]> for Data8 {
    fn from(value: [u8; 1]) -> Self {
        Self(value)
    }
}

impl From<Data8> for [u8; 1] {
    fn from(value: Data8) -> Self {
        value.0
    }
}

impl From<u8> for Data8 {
    fn from(value: u8) -> Self {
        Self([value])
    }
}

impl From<Data8> for u8 {
    fn from(value: Data8) -> Self {
        value.0[0]
    }
}
