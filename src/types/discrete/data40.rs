use le_stream::derive::{FromLeStream, ToLeStream};

/// The `40-bit data` type, short `data40`.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
#[repr(transparent)]
pub struct Data40([u8; 5]);

impl AsRef<[u8]> for Data40 {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsMut<[u8]> for Data40 {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl From<[u8; 5]> for Data40 {
    fn from(value: [u8; 5]) -> Self {
        Self(value)
    }
}

impl From<Data40> for [u8; 5] {
    fn from(value: Data40) -> Self {
        value.0
    }
}
