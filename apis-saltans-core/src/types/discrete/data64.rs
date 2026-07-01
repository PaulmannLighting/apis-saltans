use le_stream::{FromLeStream, ToLeStream};

/// The `64-bit data` type, short `data64`.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
#[repr(transparent)]
pub struct Data64([u8; 8]);

impl AsRef<[u8]> for Data64 {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsMut<[u8]> for Data64 {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl From<[u8; 8]> for Data64 {
    fn from(value: [u8; 8]) -> Self {
        Self(value)
    }
}

impl From<Data64> for [u8; 8] {
    fn from(value: Data64) -> Self {
        value.0
    }
}
