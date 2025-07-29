use alloc::boxed::Box;
use alloc::string::ToString;
use core::str;
use core::str::Utf8Error;

use le_stream::derive::{FromLeStream, ToLeStream};

use crate::types::composite::oct_str::OctStr;

/// A string type, which can be up to [`u8::MAX`] bytes long.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
#[repr(transparent)]
pub struct String(OctStr);

impl String {
    /// Try to parse the underlying bytes as a UTF-8 string.
    ///
    /// # Errors
    ///
    /// If the bytes are not valid UTF-8, this will return an [`Utf8Error`].
    pub fn try_as_str(&self) -> Result<&str, Utf8Error> {
        str::from_utf8(self.0.as_ref())
    }
}

impl AsRef<[u8]> for String {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl TryFrom<alloc::string::String> for String {
    type Error = Box<[u8]>;

    fn try_from(value: alloc::string::String) -> Result<Self, Self::Error> {
        OctStr::try_from(value.into_bytes()).map(Self)
    }
}

impl TryFrom<&str> for String {
    type Error = Box<[u8]>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_string())
    }
}
