//! Distinct type for a unicast frame.

use le_stream::{FromLeStream, ToLeStream};

pub use self::header::Header;

mod header;

/// An APS unicast frame.
#[derive(Clone, Debug, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
pub struct Unicast<T> {
    header: Header,
    payload: T,
}

impl<T> Unicast<T> {
    /// Create a new unicast frame.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the provided header is consistent with the payload.
    #[expect(unsafe_code)]
    #[must_use]
    pub const unsafe fn new(header: Header, payload: T) -> Self {
        Self { header, payload }
    }

    /// Return a reference to the frame's header.
    #[must_use]
    pub const fn header(&self) -> &Header {
        &self.header
    }

    /// Return a reference to the frame's payload.
    #[must_use]
    pub const fn payload(&self) -> &T {
        &self.payload
    }

    /// Return the frame's header and payload, consuming the frame.
    #[must_use]
    pub fn into_parts(self) -> (Header, T) {
        (self.header, self.payload)
    }
}
