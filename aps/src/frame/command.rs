//! APS Command Frame.

use le_stream::{FromLeStream, ToLeStream};

pub use self::header::Header;
use crate::Control;

mod header;

/// APS Command Frame.
#[derive(Clone, Debug, Eq, Hash, PartialEq, ToLeStream)]
pub struct Frame<T> {
    header: Header,
    payload: T,
}

impl<T> Frame<T> {
    /// Create a new command frame.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the provided header is consistent with the payload.
    #[expect(unsafe_code)]
    #[must_use]
    pub const unsafe fn new_unchecked(header: Header, payload: T) -> Self {
        Self { header, payload }
    }

    /// Return a reference to the header.
    #[must_use]
    pub const fn header(&self) -> &Header {
        &self.header
    }

    /// Return a reference to the payload.
    #[must_use]
    pub const fn payload(&self) -> &T {
        &self.payload
    }

    /// Return the header and payload consuming the frame.
    #[must_use]
    pub fn into_parts(self) -> (Header, T) {
        (self.header, self.payload)
    }
}

impl<T> Frame<T>
where
    T: FromLeStream,
{
    /// Creates a new APS Command frame from a little-endian byte stream with the given control field.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the control field indicates a valid Command frame.
    #[expect(unsafe_code)]
    pub unsafe fn from_le_stream_with_control<U>(control: Control, mut bytes: U) -> Option<Self>
    where
        U: Iterator<Item = u8>,
    {
        let counter = u8::from_le_stream(&mut bytes)?;
        let id = u8::from_le_stream(&mut bytes)?;
        let payload = T::from_le_stream(&mut bytes)?;

        Some(Self {
            header: Header::new(control, counter, id),
            payload,
        })
    }
}
