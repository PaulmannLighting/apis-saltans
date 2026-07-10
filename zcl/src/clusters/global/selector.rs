//! Structured attribute selector.

use std::boxed::Box;
use std::vec::Vec;

use le_stream::{FromLeStream, ToLeStream};

const LIMIT: usize = 0x0f;
const INDICATOR_OFFSET: usize = 4;

/// Selector for structured attribute access.
///
/// The selector contains the nested element indices of an array, structure, set,
/// or bag. For structured write commands, `operation` carries the upper nibble
/// of the selector indicator: `0` writes the selected element, `1` adds an
/// element to a set or bag, and `2` removes an element from a set or bag.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Selector {
    operation: u8,
    indexes: Box<[u16]>,
}

impl Selector {
    /// Create a selector for a whole attribute.
    #[must_use]
    pub fn whole() -> Self {
        Self {
            operation: 0,
            indexes: Box::new([]),
        }
    }

    /// Create a structured attribute selector.
    ///
    /// Returns `None` if more than 15 nested indices are supplied.
    #[must_use]
    pub fn new(operation: u8, indexes: Box<[u16]>) -> Option<Self> {
        (indexes.len() <= LIMIT).then_some(Self { operation, indexes })
    }

    /// Return the structured write operation.
    #[must_use]
    pub const fn operation(&self) -> u8 {
        self.operation
    }

    /// Return the nested element indices.
    #[must_use]
    pub fn indexes(&self) -> &[u16] {
        &self.indexes
    }
}

impl FromLeStream for Selector {
    fn from_le_stream<T>(mut bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        let indicator = u8::from_le_stream(&mut bytes)?;
        let operation = indicator >> INDICATOR_OFFSET;
        let count = usize::from(indicator) & LIMIT;
        let mut indexes = Vec::with_capacity(count);

        for _ in 0..count {
            indexes.push(u16::from_le_stream(&mut bytes)?);
        }

        Some(Self {
            operation,
            indexes: indexes.into_boxed_slice(),
        })
    }
}

impl ToLeStream for Selector {
    type Iter = <Vec<u8> as IntoIterator>::IntoIter;

    fn to_le_stream(self) -> Self::Iter {
        let count = u8::try_from(self.indexes.len())
            .expect("selector constructor limits index count to 15");
        let indicator = (self.operation << INDICATOR_OFFSET) | count;
        let mut bytes = Vec::with_capacity(usize::from(1 + count) * 2);

        bytes.push(indicator);
        bytes.extend(self.indexes.into_iter().flat_map(ToLeStream::to_le_stream));
        bytes.into_iter()
    }
}
