//! ZCL frame representation.

pub use header::{Control, Direction, Header, Type};

use crate::zcl::Command;

mod header;

/// A ZCL frame.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Frame<T> {
    header: Header,
    payload: T,
}

impl<T> Frame<T>
where
    T: Command,
{
    /// Create a new ZCL frame.
    #[must_use]
    pub fn new(
        typ: Type,
        direction: Direction,
        disable_client_response: bool,
        manufacturer_code: Option<u16>,
        seq: u8,
        payload: T,
    ) -> Self {
        Self {
            header: Header::new(
                typ,
                direction,
                disable_client_response,
                manufacturer_code,
                seq,
                <T as Command>::ID,
            ),
            payload,
        }
    }

    /// Return the header of the ZCL frame.
    #[must_use]
    pub const fn header(&self) -> &Header {
        &self.header
    }

    /// Return the payload of the ZCL frame.
    #[must_use]
    pub const fn payload(&self) -> &T {
        &self.payload
    }

    /// Consume the frame and return its header.
    #[must_use]
    pub fn into_header(self) -> Header {
        self.header
    }

    /// Consume the frame and return its payload.
    #[must_use]
    pub fn into_payload(self) -> T {
        self.payload
    }

    /// Consume the frame and return its header and payload.
    #[must_use]
    pub fn into_parts(self) -> (Header, T) {
        (self.header, self.payload)
    }
}
