use core::fmt::Debug;
use core::iter::Chain;
use core::marker::PhantomData;

use le_stream::{FromLeStream, ToLeStream};

use crate::constants::U8_CAPACITY;
use crate::types::Uint8;

/// A list of items with a length prefix.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct List<P, T, const CAPACITY: usize = U8_CAPACITY> {
    items: heapless::Vec<T, CAPACITY>,
    prefix: PhantomData<P>,
}

impl<T, const CAPACITY: usize> List<Uint8, T, CAPACITY> {
    /// Creates a new `List` with the specified items.
    #[must_use]
    pub fn new(items: heapless::Vec<T, CAPACITY>) -> Option<Self> {
        u8::try_from(items.len())
            .ok()
            .try_into()
            .ok()
            .map(|_: Uint8| Self {
                items,
                prefix: PhantomData,
            })
    }
}

impl<P, T, const CAPACITY: usize> List<P, T, CAPACITY> {
    /// Return the inner `Box<[T]>` of the list.
    #[must_use]
    pub fn into_inner(self) -> heapless::Vec<T, CAPACITY> {
        self.items
    }
}

impl<P, T, const CAPACITY: usize> AsRef<[T]> for List<P, T, CAPACITY> {
    fn as_ref(&self) -> &[T] {
        self.items.as_ref()
    }
}

impl<P, T, const CAPACITY: usize> AsMut<[T]> for List<P, T, CAPACITY> {
    fn as_mut(&mut self) -> &mut [T] {
        self.items.as_mut()
    }
}

impl<T, const CAPACITY: usize> FromLeStream for List<Uint8, T, CAPACITY>
where
    T: FromLeStream,
{
    fn from_le_stream<I>(mut bytes: I) -> Option<Self>
    where
        I: Iterator<Item = u8>,
    {
        // If the prefix is `None`, i.e. an invalid read, we assume an empty list.
        let size = Option::<u8>::from(Uint8::from_le_stream(&mut bytes)?).unwrap_or_default();
        let mut items = heapless::Vec::new();

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

impl<T, const CAPACITY: usize> ToLeStream for List<Uint8, T, CAPACITY>
where
    T: ToLeStream,
{
    type Iter =
        Chain<<Uint8 as ToLeStream>::Iter, <heapless::Vec<T, CAPACITY> as ToLeStream>::Iter>;

    fn to_le_stream(self) -> Self::Iter {
        Uint8::try_from(u8::try_from(self.items.len()).ok())
            .expect("List length should be a valid Uint8.")
            .to_le_stream()
            .chain(self.items.to_le_stream())
    }
}
