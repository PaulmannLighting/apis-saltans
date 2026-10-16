//! APS Data frame definitions.

use core::num::NonZero;

use le_stream::{FromLeStream, ToLeStream};

pub use self::defragmentation::{Assembler, AssemblyError};
pub use self::fragments::{FragmentationError, Fragments};
pub use self::header::{DataHeaderParameters, Header};
pub use self::unicast::{Unicast, UnicastHeaderParameters};
use crate::Payload;

mod defragmentation;
mod fragments;
mod header;
#[cfg(test)]
mod tests;
mod unicast;

/// An APS Data frame.
#[derive(Clone, Debug, Eq, PartialEq, Hash, ToLeStream)]
pub struct Frame<T> {
    header: Header,
    payload: T,
}

impl<T> Frame<T> {
    /// Creates a new APS Data frame without any validation.
    #[must_use]
    pub const fn new(header: Header, payload: T) -> Self {
        Self { header, payload }
    }

    /// Return a reference to the header.
    #[must_use]
    pub const fn header(&self) -> Header {
        self.header
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
    pub fn into_parts(self) -> (Header, T) {
        (self.header, self.payload)
    }

    /// Transform the payload while preserving the APS data header.
    #[must_use]
    pub fn map_payload<U, F>(self, map: F) -> Frame<U>
    where
        F: FnOnce(T) -> U,
    {
        Frame {
            header: self.header,
            payload: map(self.payload),
        }
    }
}

impl Frame<Payload> {
    /// Return a new frame with the given header and payload.
    #[must_use]
    pub const fn raw(header: Header, payload: Payload) -> Self {
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
    /// extended header block count field, or an allocation-free capacity is exceeded.
    ///
    pub fn fragment(self, chunk_size: NonZero<usize>) -> Result<Fragments, FragmentationError> {
        Fragments::new(self, chunk_size)
    }
}

impl Frame<Payload> {
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
