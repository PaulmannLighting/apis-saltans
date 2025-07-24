use core::str;
use core::str::Utf8Error;

use le_stream::derive::{FromLeStream, ToLeStream};

use super::oct_str16::OctStr16;

/// A string type, which can be up to [`u16::MAX`] bytes long.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
#[repr(transparent)]
pub struct String16(OctStr16);

impl String16 {
    /// Try to parse the underlying bytes as a UTF-8 string.
    pub fn try_as_str(&self) -> Result<&str, Utf8Error> {
        str::from_utf8(self.0.as_ref())
    }
}

impl AsRef<str> for String16 {
    fn as_ref(&self) -> &str {
        self.try_as_str().unwrap_or_default()
    }
}

impl AsRef<[u8]> for String16 {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl TryFrom<&str> for String16 {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        OctStr16::try_from(value.as_bytes()).map(Self)
    }
}
