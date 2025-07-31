use core::str;
use core::str::Utf8Error;

use le_stream::derive::{FromLeStream, ToLeStream};

use crate::constants::U8_CAPACITY;
use crate::types::{OctStr, U8String};

/// A string type, which can be up to [`OctStr::MAX_SIZE`] bytes long.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
#[repr(transparent)]
pub struct String<const CAPACITY: usize = U8_CAPACITY>(OctStr);

impl<const CAPACITY: usize> String<CAPACITY> {
    /// Return the length in bytes.
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Determine whether the string is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Try to parse the underlying bytes as a UTF-8 string.
    ///
    /// # Errors
    ///
    /// If the bytes are not valid UTF-8, this will return an [`Utf8Error`].
    pub fn try_as_str(&self) -> Result<&str, Utf8Error> {
        str::from_utf8(self.0.as_ref())
    }
}

impl<const CAPACITY: usize> AsRef<[u8]> for String<CAPACITY> {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl<const CAPACITY: usize> From<U8String> for String<CAPACITY> {
    fn from(value: U8String) -> Self {
        Self(OctStr::from(value.into_bytes()))
    }
}

impl<const CAPACITY: usize> TryFrom<&str> for String<CAPACITY> {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        OctStr::try_from(value.as_bytes()).map(Self)
    }
}
