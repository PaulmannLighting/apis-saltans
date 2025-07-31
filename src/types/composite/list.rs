use alloc::boxed::Box;
use alloc::vec::Vec;
use core::iter::Chain;
use core::marker::PhantomData;

use le_stream::{FromLeStream, ToLeStream};

use crate::types::Uint8;

/// A list of items with a length prefix.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct List<P, T> {
    items: Box<[T]>,
    prefix: PhantomData<P>,
}

impl<T> List<Uint8, T> {
    /// Creates a new `List` with the specified items.
    #[must_use]
    pub fn new(items: Box<[T]>) -> Option<Self> {
        u8::try_from(items.len())
            .ok()
            .and_then(Uint8::new)
            .map(|_| Self {
                items,
                prefix: PhantomData,
            })
    }
}

impl<P, T> List<P, T> {
    /// Return the inner `Box<[T]>` of the list.
    #[must_use]
    pub fn into_inner(self) -> Box<[T]> {
        self.items
    }
}

impl<P, T> AsRef<[T]> for List<P, T> {
    fn as_ref(&self) -> &[T] {
        self.items.as_ref()
    }
}

impl<P, T> AsMut<[T]> for List<P, T> {
    fn as_mut(&mut self) -> &mut [T] {
        self.items.as_mut()
    }
}

impl<T> FromLeStream for List<Uint8, T>
where
    T: FromLeStream,
{
    fn from_le_stream<I>(mut bytes: I) -> Option<Self>
    where
        I: Iterator<Item = u8>,
    {
        let prefix: Uint8 = u8::from_le_stream(&mut bytes)?
            .try_into()
            .unwrap_or_default();
        let mut items = Vec::new();

        for _ in 0..Option::<u8>::from(prefix).unwrap_or_default() {
            items.push(T::from_le_stream(&mut bytes)?);
        }

        Some(Self {
            items: items.into_boxed_slice(),
            prefix: PhantomData,
        })
    }
}

impl<T> ToLeStream for List<Uint8, T>
where
    T: ToLeStream,
{
    type Iter = Chain<<Uint8 as ToLeStream>::Iter, <Box<[T]> as ToLeStream>::Iter>;

    fn to_le_stream(self) -> Self::Iter {
        u8::try_from(self.items.len())
            .ok()
            .and_then(Uint8::new)
            .expect("List length should be a valid Uint8.")
            .to_le_stream()
            .chain(self.items.to_le_stream())
    }
}
