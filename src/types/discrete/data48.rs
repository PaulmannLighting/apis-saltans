use le_stream::derive::{FromLeStream, ToLeStream};

/// The `48-bit data` type, short `data48`.
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
#[repr(transparent)]
pub struct Data48([u8; 6]);

impl AsRef<[u8]> for Data48 {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsMut<[u8]> for Data48 {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl From<[u8; 6]> for Data48 {
    fn from(value: [u8; 6]) -> Self {
        Self(value)
    }
}

impl From<Data48> for [u8; 6] {
    fn from(value: Data48) -> Self {
        value.0
    }
}
