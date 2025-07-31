use core::fmt::Debug;
use core::iter::Chain;
use core::marker::PhantomData;

use le_stream::{FromLeStream, ToLeStream};

use crate::types::Uint8;
use crate::types::misc::U8Vec;

/// A list of items with a length prefix.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct List<P, T> {
    items: U8Vec<T>,
    prefix: PhantomData<P>,
}

impl<T> List<Uint8, T> {
    /// Creates a new `List` with the specified items.
    #[must_use]
    pub fn new(items: U8Vec<T>) -> Option<Self> {
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
    pub fn into_inner(self) -> U8Vec<T> {
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
        // If the prefix is `None`, i.e. an invalid read, we assume an empty list.
        let size = Option::<u8>::from(Uint8::from_le_stream(&mut bytes)?).unwrap_or_default();
        let mut items = U8Vec::new();

        for _ in 0..size {
            // If the item cannot be added, return `None`.
            items.push(T::from_le_stream(&mut bytes)?).ok()?;
        }

        Some(Self {
            items,
            prefix: PhantomData,
        })
    }
}

impl<T> ToLeStream for List<Uint8, T>
where
    T: ToLeStream,
{
    type Iter = Chain<<Uint8 as ToLeStream>::Iter, <U8Vec<T> as ToLeStream>::Iter>;

    fn to_le_stream(self) -> Self::Iter {
        u8::try_from(self.items.len())
            .ok()
            .and_then(Uint8::new)
            .expect("List length should be a valid Uint8.")
            .to_le_stream()
            .chain(self.items.to_le_stream())
    }
}
