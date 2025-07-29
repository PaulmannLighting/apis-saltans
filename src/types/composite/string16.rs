use alloc::boxed::Box;
use alloc::string::{String, ToString};
use core::str;
use core::str::Utf8Error;

use le_stream::derive::{FromLeStream, ToLeStream};

use crate::types::composite::oct_str16::OctStr16;

/// A string type, which can be up to [`OctStr16::MAX_SIZE`] bytes long.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
#[repr(transparent)]
pub struct String16(OctStr16);

impl String16 {
    /// Try to parse the underlying bytes as a UTF-8 string.
    ///
    /// # Errors
    ///
    /// If the bytes are not valid UTF-8, this will return an [`Utf8Error`].
    pub fn try_as_str(&self) -> Result<&str, Utf8Error> {
        str::from_utf8(self.0.as_ref())
    }
}

impl AsRef<[u8]> for String16 {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl TryFrom<String> for String16 {
    type Error = Box<[u8]>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        OctStr16::try_from(value.into_bytes()).map(Self)
    }
}

impl TryFrom<&str> for String16 {
    type Error = Box<[u8]>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_string())
    }
}
