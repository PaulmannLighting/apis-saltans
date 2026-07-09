//! APS Data frame definitions.

use std::num::{NonZero, TryFromIntError};

use bytes::Bytes;
use le_stream::{FromLeStream, ToLeStream};

pub use self::defragmentation::Assembler;
pub use self::fragments::Fragments;
pub use self::header::Header;
pub use self::unicast::Unicast;
use crate::Destination;

mod defragmentation;
mod fragments;
mod header;
mod unicast;

/// An APS Data frame.
#[derive(Clone, Debug, Eq, PartialEq, Hash, ToLeStream)]
pub struct Frame<T, D = Destination> {
    header: Header<D>,
    payload: T,
}

impl<T, D> Frame<T, D> {
    /// Creates a new APS Data frame without any validation.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the provided header is consistent with the payload.
    #[expect(unsafe_code)]
    #[must_use]
    pub const unsafe fn new_unchecked(header: Header<D>, payload: T) -> Self {
        Self { header, payload }
    }

    /// Return a reference to the header.
    #[must_use]
    pub const fn header(&self) -> &Header<D> {
        &self.header
    }

    /// Return a reference to the payload.
    #[must_use]
    pub const fn payload(&self) -> &T {
        &self.payload
    }

    /// Drop the extended header.
    pub fn drop_extended(&mut self) {
        self.header.drop_extended();
    }

    /// Return the header and payload, consuming the frame.
    #[must_use]
    pub fn into_parts(self) -> (Header<D>, T) {
        (self.header, self.payload)
    }

    /// Convert the frame to use the default APS destination representation.
    ///
    /// This keeps the payload and all header fields unchanged except for the
    /// destination, which is converted into [`Destination`].
    #[must_use]
    pub fn into_default_dst(self) -> Frame<T, Destination>
    where
        D: Into<Destination>,
    {
        let (header, payload) = self.into_parts();
        Frame::<T, Destination> {
            header: header.into_default_dst(),
            payload,
        }
    }
}

impl<D> Frame<Bytes, D> {
    /// Return a new frame with the given header and payload.
    #[must_use]
    pub const fn raw(header: Header<D>, payload: Bytes) -> Self {
        Self { header, payload }
    }

    /// Fragment the frame payload into APS data frames of at most `chunk_size` bytes.
    ///
    /// The returned iterator owns the frame payload and yields frames with extended
    /// headers that describe the first and follow-up fragments.
    ///
    /// # Errors
    ///
    /// Returns an error if the number of fragments does not fit into the APS
    /// extended header block count field.
    ///
    pub fn fragment(self, chunk_size: NonZero<usize>) -> Result<Fragments<D>, TryFromIntError>
    where
        D: Copy,
    {
        Fragments::new(self, chunk_size)
    }
}

impl Frame<Bytes> {
    /// Parse the frame into a frame with typed payload.
    ///
    /// # Errors
    ///
    /// Returns an error if the payload cannot be parsed into the given type.
    pub fn parse<T>(self) -> Result<Frame<T>, T::Error>
    where
        T: TryFrom<Self>,
    {
        let header = self.header;
        T::try_from(self).map(|payload| Frame { header, payload })
    }
}

impl<T> From<Unicast<T>> for Frame<T> {
    fn from(unicast: Unicast<T>) -> Self {
        let (header, payload) = unicast.into_parts();

        Self {
            header: header.into(),
            payload,
        }
    }
}

impl<T> FromLeStream for Frame<T>
where
    T: FromLeStream,
{
    fn from_le_stream<I>(mut bytes: I) -> Option<Self>
    where
        I: Iterator<Item = u8>,
    {
        let header = Header::from_le_stream(&mut bytes)?;
        let payload = T::from_le_stream(&mut bytes)?;
        Some(Self { header, payload })
    }
}
