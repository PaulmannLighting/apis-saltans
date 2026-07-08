//! ZCL frame representation.

use apis_saltans_aps::Data;
use bytes::Bytes;
use le_stream::{FromLeStream, ToLeStream};

pub use self::header::{Control, Direction, Header, Scope};
pub use self::parse_frame_error::ParseFrameError;
use crate::CommandDispatch;
use crate::clusters::Cluster;

mod header;
mod parse_frame_error;

/// A ZCL frame.
#[derive(Clone, Debug, Eq, Hash, PartialEq, FromLeStream, ToLeStream)]
pub struct Frame<T> {
    header: Header,
    payload: T,
}

impl<T> Frame<T> {
    /// Create a new ZCL frame from the given header and payload.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the provided header and payload are consistent.
    #[expect(unsafe_code)]
    #[must_use]
    pub const unsafe fn new_unchecked(header: Header, payload: T) -> Self {
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
}

/// A parsed ZCL frame.
impl Frame<Cluster> {
    #[must_use]
    pub fn new(seq: u8, manufacturer_code: Option<u16>, payload: Cluster) -> Frame<Cluster> {
        let header = Header::new(
            payload.scope(),
            payload.direction(),
            payload.disable_default_response(),
            manufacturer_code,
            seq,
            payload.command_id(),
        );
        Self { header, payload }
    }

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
