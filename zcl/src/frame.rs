//! ZCL frame representation.

use bytes::Bytes;
use le_stream::{FromLeStream, ToLeStream};
use zb_aps::Data;

pub use self::header::{Control, Direction, Header, Scope};
pub use self::parse_frame_error::ParseFrameError;
pub use self::unsequenced::{UnsequencedFrame, UnsequencedHeader};
use crate::Cluster;

mod header;
mod parse_frame_error;
mod unsequenced;

/// A ZCL frame.
#[derive(Clone, Debug, Eq, Hash, PartialEq, FromLeStream, ToLeStream)]
pub struct Frame<T> {
    header: Header,
    payload: T,
}

impl<T> Frame<T> {
    /// Create a ZCL frame from a header and its payload.
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

    /// Override whether the frame disables the default response.
    #[must_use]
    pub fn with_disable_default_response(mut self, disabled: bool) -> Self {
        self.header.set_disable_default_response(disabled);
        self
    }

    /// Transform the payload while preserving the ZCL header.
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

/// A parsed ZCL frame.
impl Frame<Cluster> {
    /// Parse a ZCL frame from a little-endian byte stream.
    ///
    /// # Errors
    ///
    /// Returns [`ParseFrameError`] if the frame cannot be parsed.
    pub fn parse<T>(cluster_id: u16, mut bytes: T) -> Result<Self, ParseFrameError>
    where
        T: Iterator<Item = u8>,
    {
        let header = Header::from_le_stream(&mut bytes).ok_or(ParseFrameError::MissingHeader)?;
        let payload = Cluster::parse_zcl_cluster(cluster_id, header, bytes)?;
        Ok(Self { header, payload })
    }
}

impl TryFrom<Data<Bytes>> for Frame<Cluster> {
    type Error = ParseFrameError;

    fn try_from(frame: Data<Bytes>) -> Result<Self, Self::Error> {
        let (header, payload) = frame.into_parts();
        Self::parse(header.cluster_id(), payload.into_iter())
    }
}

#[cfg(test)]
mod tests {
    use bytes::Bytes;

    use super::{Direction, Frame, Header, Scope};

    const SEQUENCE_NUMBER: u8 = 42;
    const COMMAND_ID: u8 = 1;

    #[test]
    fn overrides_default_response_flag() {
        let frame = Frame::new(
            Header::new(
                Scope::Global,
                Direction::ClientToServer,
                true,
                None,
                SEQUENCE_NUMBER,
                COMMAND_ID,
            ),
            Bytes::new(),
        )
        .with_disable_default_response(false);

        assert!(!frame.header().control().disable_default_response());
    }
}
