use le_stream::derive::{FromLeStream, ToLeStream};

/// The `16-bit data` type, short `data16`.
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
#[repr(transparent)]
pub struct Data16([u8; 2]);

impl AsRef<[u8]> for Data16 {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsMut<[u8]> for Data16 {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl From<[u8; 2]> for Data16 {
    fn from(value: [u8; 2]) -> Self {
        Self(value)
    }
}

impl From<Data16> for [u8; 2] {
    fn from(value: Data16) -> Self {
        value.0
    }
}
