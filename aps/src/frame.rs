use le_stream::{FromLeStream, ToLeStream};

pub use self::header::{
    Acknowledgment, Command, Control, Data, DeliveryMode, Destination, Extended, FrameType, Header,
};

mod header;

/// APS Frame consisting of a header and a payload.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
pub struct Frame<T> {
    header: Header,
    payload: T,
}

impl<T> Frame<T> {
    /// Creates a new APS Frame with the given header and payload.
    #[must_use]
    pub const fn new(header: Header, payload: T) -> Self {
        Self { header, payload }
    }

    /// Returns a reference to the APS Frame header.
    #[must_use]
    pub const fn header(&self) -> &Header {
        &self.header
    }

    /// Returns a reference to the APS Frame payload.
    #[must_use]
    pub const fn payload(&self) -> &T {
        &self.payload
    }
}
