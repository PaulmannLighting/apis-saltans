use le_stream::{FromLeStream, ToLeStream};

/// The `32-bit data` type, short `data32`.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
#[repr(transparent)]
pub struct Data32([u8; 4]);

impl AsRef<[u8]> for Data32 {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsMut<[u8]> for Data32 {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl From<[u8; 4]> for Data32 {
    fn from(value: [u8; 4]) -> Self {
        Self(value)
    }
}

impl From<Data32> for [u8; 4] {
    fn from(value: Data32) -> Self {
        value.0
    }
}
