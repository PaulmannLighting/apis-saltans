use core::iter::Chain;

use le_stream::{FromLeStream, ToLeStream};

use crate::types::tlv::Tag;

const LENGTH_OFFSET: usize = 1;

/// The raw payload of a general TLV.
pub type Payload = heapless::Vec<u8, { u8::MAX as usize + LENGTH_OFFSET }, usize>;

/// A general TLV.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct General {
    typ: u8,
    payload: Payload,
}

impl General {
    /// Create a new general TLV.
    #[must_use]
    pub const fn new(typ: u8, payload: Payload) -> Self {
        Self { typ, payload }
    }

    /// Serialize a typed TLV into a general TLV.
    #[must_use]
    pub fn serialize<T>(tlv: T) -> Self
    where
        T: Tag + ToLeStream,
    {
        Self {
            typ: T::TAG,
            payload: tlv.to_le_stream().collect(),
        }
    }

    /// Returns the type ID.
    #[must_use]
    pub const fn typ(&self) -> u8 {
        self.typ
    }

    /// Return the constituent parts, consuming general TLV.
    #[must_use]
    pub fn into_parts(self) -> (u8, Payload) {
        (self.typ, self.payload)
    }
}

/// Create  a new `General` TLV.
///
/// This assumes that we already consumed the TAG.
impl FromLeStream for General {
    fn from_le_stream<T>(mut bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        let typ = u8::from_le_stream(&mut bytes)?;
        let length = usize::from(u8::from_le_stream(&mut bytes)?) + LENGTH_OFFSET;

        let mut payload = Payload::new();

        for _ in 0..length {
            payload.push(u8::from_le_stream(&mut bytes)?).ok()?;
        }

        Some(Self { typ, payload })
    }
}

impl ToLeStream for General {
    type Iter = Chain<
        Chain<<u8 as ToLeStream>::Iter, <u8 as ToLeStream>::Iter>,
        <Payload as IntoIterator>::IntoIter,
    >;

    fn to_le_stream(self) -> Self::Iter {
        let length: u8 = self
            .payload
            .len()
            .checked_sub(LENGTH_OFFSET)
            .expect("Length of General TLV is guaranteed to be > 0.")
            .try_into()
            .expect("Length of General TLV is guaranteed to fit in a u8.");
        self.typ
            .to_le_stream()
            .chain(length.to_le_stream())
            .chain(self.payload.into_iter())
    }
}
