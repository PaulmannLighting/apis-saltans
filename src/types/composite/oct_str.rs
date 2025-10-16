use core::iter::Chain;

use le_stream::{FromLeStream, ToLeStream};
use log::{error, warn};

use crate::constants::U8_CAPACITY;
use crate::types::Uint8;

/// An octet string with a maximum size of [`OctStr::CAPACITY`] bytes.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct OctStr<const CAPACITY: usize = U8_CAPACITY>(heapless::Vec<u8, CAPACITY>);

impl<const CAPACITY: usize> OctStr<CAPACITY> {
    /// The maximum size of an `OctStr` in bytes.
    pub const CAPACITY: usize = CAPACITY;

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
}

impl<const CAPACITY: usize> AsRef<[u8]> for OctStr<CAPACITY> {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl<const CAPACITY: usize> AsMut<[u8]> for OctStr<CAPACITY> {
    fn as_mut(&mut self) -> &mut [u8] {
        self.0.as_mut()
    }
}

impl<const CAPACITY: usize> From<heapless::Vec<u8, CAPACITY>> for OctStr<CAPACITY> {
    fn from(bytes: heapless::Vec<u8, CAPACITY>) -> Self {
        Self(bytes)
    }
}

impl<const CAPACITY: usize> From<heapless::String<CAPACITY>> for OctStr<CAPACITY> {
    fn from(string: heapless::String<CAPACITY>) -> Self {
        Self::from(string.into_bytes())
    }
}

impl<const CAPACITY: usize> TryFrom<&[u8]> for OctStr<CAPACITY> {
    type Error = ();

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() > Self::CAPACITY {
            return Err(());
        }

        Ok(Self::from(
            value
                .iter()
                .copied()
                .collect::<heapless::Vec<u8, CAPACITY>>(),
        ))
    }
}

impl<const CAPACITY: usize> FromLeStream for OctStr<CAPACITY> {
    // TODO: For the sake of robustness, reconsider handling of the case `size > CAPACITY`.
    #[allow(clippy::unwrap_in_result)]
    fn from_le_stream<T>(mut stream: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        let size: u8 = Option::<u8>::from(Uint8::from_le_stream(&mut stream)?).unwrap_or(0);

        if CAPACITY < size.into() {
            warn!("OctStr size {size} exceeds maximum capacity of {CAPACITY} bytes.");
        }

        let mut bytes = heapless::Vec::new();

        for _ in 0..size {
            bytes
                .push(u8::from_le_stream(&mut stream)?)
                .unwrap_or_else(|byte| error!("Buffer is full. Discarding byte: {byte:#04X}"));
        }

        Some(Self(bytes))
    }
}

impl<const CAPACITY: usize> ToLeStream for OctStr<CAPACITY> {
    type Iter =
        Chain<<Uint8 as ToLeStream>::Iter, <heapless::Vec<u8, CAPACITY> as IntoIterator>::IntoIter>;

    fn to_le_stream(self) -> Self::Iter {
        Uint8::try_from(u8::try_from(self.0.len()).expect("Length should fit into u8."))
            .expect("Length should be a valid Uint8.")
            .to_le_stream()
            .chain(self.0)
    }
}
