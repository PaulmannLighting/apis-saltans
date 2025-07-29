use alloc::boxed::Box;
use alloc::vec::Vec;
use core::iter::Chain;

use le_stream::{FromLeStream, ToLeStream};

use crate::types::Uint16;

/// An octet string, with a capacity of [`OctStr16::MAX_SIZE`].
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct OctStr16(Box<[u8]>);

impl OctStr16 {
    /// Maximum size of the octet string.
    pub const MAX_SIZE: u16 = Uint16::NON_VALUE - 1;
}

impl AsRef<[u8]> for OctStr16 {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl AsMut<[u8]> for OctStr16 {
    fn as_mut(&mut self) -> &mut [u8] {
        self.0.as_mut()
    }
}

impl TryFrom<Box<[u8]>> for OctStr16 {
    type Error = Box<[u8]>;

    /// Try to create an `OctStr16` from a boxed slice of bytes.
    ///
    /// # Errors
    ///
    /// If the length of the boxed slice exceeds [`Self::MAX_SIZE`], this will return an error with the original value.
    fn try_from(value: Box<[u8]>) -> Result<Self, Self::Error> {
        if u16::try_from(value.len())
            .ok()
            .and_then(Uint16::new)
            .is_some()
        {
            Ok(Self(value))
        } else {
            Err(value)
        }
    }
}

impl TryFrom<Vec<u8>> for OctStr16 {
    type Error = Box<[u8]>;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Self::try_from(value.into_boxed_slice())
    }
}

impl FromLeStream for OctStr16 {
    fn from_le_stream<T>(mut bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        let size: u16 = Option::<u16>::from(Uint16::from_le_stream(&mut bytes)?).unwrap_or(0);
        let mut data = Vec::with_capacity(usize::from(size));

        for _ in 0..size {
            data.push(u8::from_le_stream(&mut bytes)?);
        }

        Some(Self(data.into_boxed_slice()))
    }
}

impl ToLeStream for OctStr16 {
    type Iter = Chain<<Uint16 as ToLeStream>::Iter, <Box<[u8]> as IntoIterator>::IntoIter>;

    fn to_le_stream(self) -> Self::Iter {
        let size = Uint16::new(u16::try_from(self.0.len()).expect("Length should fit into u16."))
            .expect("Length should be a valid Uint16.");
        size.to_le_stream().chain(self.0.into_iter())
    }
}
