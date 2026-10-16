use core::fmt::{Debug, Formatter};
use core::iter::once;
use core::ops::Deref;

use heapless::Vec;
use le_stream::{FromLeStream, ToLeStream};

use super::CapacityError;

const PREFIX_CAPACITY: usize = 1;

/// A bounded sequence encoded without a length prefix.
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent)
)]
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct RawList<T, const N: usize>(Vec<T, N>);

impl<T, const N: usize> Default for RawList<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const N: usize> RawList<T, N> {
    /// Create an empty bounded sequence.
    #[must_use]
    pub const fn new() -> Self {
        Self(Vec::new())
    }
    /// Borrow the stored elements.
    #[must_use]
    pub fn as_slice(&self) -> &[T] {
        self.0.as_slice()
    }

    /// Append an element, returning it if the capacity is exhausted.
    ///
    /// # Errors
    /// Returns the supplied element when full.
    pub fn push(&mut self, value: T) -> Result<(), T> {
        self.0.push(value)
    }

    /// Collect a sequence, rejecting excess elements.
    ///
    /// # Errors
    /// Returns a capacity error if the sequence is too long.
    pub fn try_from_iter<I>(items: I) -> Result<Self, CapacityError>
    where
        I: IntoIterator<Item = T>,
    {
        let mut result = Self::default();
        for item in items {
            result
                .push(item)
                .map_err(|_| CapacityError { capacity: N })?;
        }
        Ok(result)
    }
}

impl<T, const N: usize> Deref for RawList<T, N> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, const N: usize> IntoIterator for RawList<T, N> {
    type Item = T;
    type IntoIter = <Vec<T, N> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T, const N: usize> FromLeStream for RawList<T, N>
where
    T: FromLeStream,
{
    fn from_le_stream<I>(mut bytes: I) -> Option<Self>
    where
        I: Iterator<Item = u8>,
    {
        let mut result = Self::default();
        while let Some(byte) = bytes.next() {
            let value = T::from_le_stream(once(byte).chain(&mut bytes))?;
            result.push(value).ok()?;
        }
        Some(result)
    }
}

impl<T, const N: usize> ToLeStream for RawList<T, N>
where
    T: ToLeStream,
{
    type Iter = Stream<<Self as IntoIterator>::IntoIter>;
    fn to_le_stream(self) -> Self::Iter {
        Stream::new(&[], self.into_iter())
    }
}

/// Streams a short header followed by an unprefixed sequence of encoded elements.
pub struct Stream<I>
where
    I: Iterator,
    I::Item: ToLeStream,
{
    prefix: <Vec<u8, PREFIX_CAPACITY> as IntoIterator>::IntoIter,
    items: I,
    current: Option<<I::Item as ToLeStream>::Iter>,
}

impl<I> Stream<I>
where
    I: Iterator,
    I::Item: ToLeStream,
{
    pub(crate) fn new(prefix: &[u8], items: I) -> Self {
        Self {
            prefix: Vec::from_slice(prefix)
                .expect("fixed protocol header fits")
                .into_iter(),
            items,
            current: None,
        }
    }
}

impl<I> Iterator for Stream<I>
where
    I: Iterator,
    I::Item: ToLeStream,
{
    type Item = u8;
    fn next(&mut self) -> Option<u8> {
        if let Some(byte) = self.prefix.next() {
            return Some(byte);
        }
        loop {
            if let Some(byte) = self.current.as_mut().and_then(Iterator::next) {
                return Some(byte);
            }
            self.current = Some(self.items.next()?.to_le_stream());
        }
    }
}

impl<I> Debug for Stream<I>
where
    I: Iterator + Debug,
    I::Item: ToLeStream,
    <I::Item as ToLeStream>::Iter: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Stream")
            .field("prefix", &self.prefix)
            .field("items", &self.items)
            .field("current", &self.current)
            .finish()
    }
}
