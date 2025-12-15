//! ZCL frame representation.

use le_stream::{FromLeStream, ToLeStream};

pub use self::header::{Control, Direction, Header, Type};
pub use self::parse_frame_error::ParseFrameError;
use crate::Command;
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

impl<T> Frame<T>
where
    T: Command,
{
    /// Create a new ZCL frame.
    #[must_use]
    pub fn new(seq: u8, payload: T) -> Self {
        Self {
            header: Header::new(
                <T as Command>::TYPE,
                <T as Command>::DIRECTION,
                <T as Command>::DISABLE_CLIENT_RESPONSE,
                <T as Command>::MANUFACTURER_CODE,
                seq,
                <T as Command>::ID,
            ),
            payload,
        }
    }

    /// Create a new ZCL frame with an unspecified sequence number.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the sequence number is set appropriately
    /// before sending the frame, as using the default sequence number of `0x00` may lead to
    /// unexpected behavior.
    #[expect(unsafe_code)]
    pub unsafe fn new_unsequenced(payload: T) -> Self {
        Self::new(0x00, payload)
    }
}

/// A parsed ZCL frame.
impl Frame<Cluster> {
    /// Parse a ZCL frame from a little-endian byte stream.
    ///
    /// # Arguments
    ///
    /// * `cluster_id` - The cluster ID to identify the cluster of the frame.
    /// * `direction` - The direction of the command (`ClientToServer` or `ServerToClient`).
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
