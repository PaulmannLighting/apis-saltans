use le_stream::{FromLeStream, ToLeStream};

/// The `24-bit data` type, short `data24`.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
#[repr(transparent)]
pub struct Data24([u8; 3]);

impl AsRef<[u8]> for Data24 {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsMut<[u8]> for Data24 {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl From<[u8; 3]> for Data24 {
    fn from(value: [u8; 3]) -> Self {
        Self(value)
    }
}

impl From<Data24> for [u8; 3] {
    fn from(value: Data24) -> Self {
        value.0
    }
}
