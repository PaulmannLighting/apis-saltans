use core::iter::{Chain, FlatMap};
use core::ops::{Deref, DerefMut};

use le_stream::{FromLeStream, ToLeStream};

use super::MAX_NESTED_TLV_LEN;

type Inner<T> = heapless::Vec<T, { MAX_NESTED_TLV_LEN as usize }, u8>;

/// A vector of TLVs.
#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TlvVec<T> {
    inner: Inner<T>,
}

impl<T> TlvVec<T> {
    /// Creates a new TLV vector.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            inner: Inner::new(),
        }
    }

    /// Returns the TLV size with is the actual length of the TLV data minus one.
    #[expect(clippy::cast_possible_truncation)]
    pub fn tlv_size(&self) -> u8 {
        self.inner.len().saturating_sub(1) as u8
    }
}

impl<T> Deref for TlvVec<T> {
    type Target = Inner<T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for TlvVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T> FromLeStream for TlvVec<T>
where
    T: FromLeStream,
{
    fn from_le_stream<I>(mut bytes: I) -> Option<Self>
    where
        I: Iterator<Item = u8>,
    {
        let size: usize = u8::from_le_stream(&mut bytes)?.into();
        let mut inner = Inner::new();

        // Use inclusive range so that the actual length is the prefixed length plus one.
        for _ in 0..=size {
            inner.push(T::from_le_stream(&mut bytes)?).ok()?;
        }

        Some(Self { inner })
    }
}

impl<T> ToLeStream for TlvVec<T>
where
    T: ToLeStream,
{
    type Iter = Chain<
        <u8 as ToLeStream>::Iter,
        FlatMap<
            <Inner<T> as IntoIterator>::IntoIter,
            <T as ToLeStream>::Iter,
            fn(T) -> <T as ToLeStream>::Iter,
        >,
    >;

    fn to_le_stream(self) -> Self::Iter {
        let mapper: fn(T) -> <T as ToLeStream>::Iter = <T as ToLeStream>::to_le_stream;
        self.tlv_size()
            .to_le_stream()
            .chain(self.inner.into_iter().flat_map(mapper))
    }
}
