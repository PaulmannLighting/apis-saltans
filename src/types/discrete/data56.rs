use le_stream::derive::{FromLeStream, ToLeStream};

/// The `56-bit data` type, short `data56`.
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
#[repr(transparent)]
pub struct Data56([u8; 7]);

impl AsRef<[u8]> for Data56 {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsMut<[u8]> for Data56 {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl From<[u8; 7]> for Data56 {
    fn from(value: [u8; 7]) -> Self {
        Self(value)
    }
}

impl From<Data56> for [u8; 7] {
    fn from(value: Data56) -> Self {
        value.0
    }
}
