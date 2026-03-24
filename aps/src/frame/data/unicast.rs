//! Distinct type for a unicast frame.

use le_stream::ToLeStream;

pub use self::header::Header;
use crate::Extended;

mod header;

/// An APS unicast frame.
#[derive(Clone, Debug, Eq, PartialEq, Hash, ToLeStream)]
pub struct Unicast<T> {
    header: Header,
    payload: T,
}

impl<T> Unicast<T> {
    /// Create a new unicast frame.
    #[expect(clippy::too_many_arguments)]
    #[must_use]
    pub fn new(
        security: bool,
        ack_request: bool,
        dst_endpoint: u8,
        cluster_id: u16,
        profile_id: u16,
        source_endpoint: u8,
        counter: u8,
        extended: Option<Extended>,
        payload: T,
    ) -> Self {
        Self {
            header: Header::new(
                security,
                ack_request,
                dst_endpoint,
                cluster_id,
                profile_id,
                source_endpoint,
                counter,
                extended,
            ),
            payload,
        }
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
