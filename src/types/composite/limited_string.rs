use alloc::boxed::Box;
use core::fmt::Debug;
use core::str::Utf8Error;

use either::{Either, Left, Right};
use le_stream::derive::{FromLeStream, ToLeStream};

use crate::types::String;

/// A limited string type that enforces a maximum length.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
#[repr(transparent)]
pub struct LimitedString<const MAX_LEN: usize>(String);

impl<const MAX_LEN: usize> LimitedString<MAX_LEN> {
    /// Creates a new `LimitedString` if the provided string's length is within the limit.
    pub const fn new(value: String) -> Result<Self, String> {
        if value.len() <= MAX_LEN {
            Ok(Self(value))
        } else {
            Err(value)
        }
    }

    /// Return the length in bytes.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.0.len()
    }

    /// Try to parse the underlying bytes as a UTF-8 string.
    ///
    /// # Errors
    ///
    /// If the bytes are not valid UTF-8, this will return an [`Utf8Error`].
    pub fn try_as_str(&self) -> Result<&str, Utf8Error> {
        self.0.try_as_str()
    }
}

impl<const MAX_LEN: usize> AsRef<[u8]> for LimitedString<MAX_LEN> {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl<const MAX_LEN: usize> From<LimitedString<MAX_LEN>> for String {
    fn from(value: LimitedString<MAX_LEN>) -> Self {
        value.0
    }
}

impl<const MAX_LEN: usize> TryFrom<String> for LimitedString<MAX_LEN> {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl<const MAX_LEN: usize> TryFrom<&str> for LimitedString<MAX_LEN> {
    type Error = Either<Box<[u8]>, String>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(String::try_from(value).map_err(Left)?).map_err(Right)
    }
}

impl<const MAX_LEN: usize> TryFrom<alloc::string::String> for LimitedString<MAX_LEN> {
    type Error = Either<Box<[u8]>, String>;

    fn try_from(value: alloc::string::String) -> Result<Self, Self::Error> {
        Self::new(String::try_from(value).map_err(Left)?).map_err(Right)
    }
}
