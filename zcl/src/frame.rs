//! ZCL aps representation.

use le_stream::{FromLeStream, ToLeStream};
use zigbee::ClusterId;

pub use self::header::{Control, Direction, Header, Scope};
pub use self::parse_frame_error::ParseFrameError;
use crate::Command;
use crate::clusters::Cluster;

mod header;
mod parse_frame_error;

/// A ZCL aps.
#[derive(Clone, Debug, Eq, Hash, PartialEq, FromLeStream, ToLeStream)]
pub struct Frame<T> {
    header: Header,
    payload: T,
}

impl<T> Frame<T> {
    /// Create a new ZCL aps from the given header and payload.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the provided header and payload are consistent.
    #[expect(unsafe_code)]
    #[must_use]
    pub const unsafe fn new_unchecked(header: Header, payload: T) -> Self {
        Self { header, payload }
    }

    /// Return the header of the ZCL aps.
    #[must_use]
    pub const fn header(&self) -> &Header {
        &self.header
    }

    /// Return the payload of the ZCL aps.
    #[must_use]
    pub const fn payload(&self) -> &T {
        &self.payload
    }

    /// Consume the aps and return its header.
    #[must_use]
    pub fn into_header(self) -> Header {
        self.header
    }

    /// Consume the aps and return its payload.
    #[must_use]
    pub fn into_payload(self) -> T {
        self.payload
    }

    /// Consume the aps and return its header and payload.
    #[must_use]
    pub fn into_parts(self) -> (Header, T) {
        (self.header, self.payload)
    }
}

impl<T> Frame<T>
where
    T: Command,
{
    /// Create a new ZCL aps.
    #[must_use]
    pub fn new(seq: u8, payload: T) -> Self {
        Self {
            header: Header::new(
                <T as Command>::SCOPE,
                <T as Command>::DIRECTION,
                <T as Command>::DISABLE_CLIENT_RESPONSE,
                <T as Command>::MANUFACTURER_CODE,
                seq,
                <T as Command>::ID,
            ),
            payload,
        }
    }
}

/// A parsed ZCL aps.
impl Frame<Cluster> {
    /// Parse a ZCL aps from a little-endian byte stream.
    ///
    /// # Arguments
    ///
    /// * `cluster_id` - The cluster ID to identify the cluster of the aps.
    /// * `direction` - The direction of the command (`ClientToServer` or `ServerToClient`).
    ///
    /// # Errors
    ///
    /// Returns [`ParseFrameError`] if the aps cannot be parsed.
    pub fn parse<T>(cluster_id: u16, mut bytes: T) -> Result<Self, ParseFrameError>
    where
        T: Iterator<Item = u8>,
    {
        let header = Header::from_le_stream(&mut bytes).ok_or(ParseFrameError::MissingHeader)?;
        let payload = Cluster::parse_zcl_cluster(cluster_id, header, bytes)?;
        Ok(Self { header, payload })
    }
}

impl<T> ClusterId for Frame<T>
where
    T: ClusterId,
{
    fn cluster_id(&self) -> u16 {
        self.payload.cluster_id()
    }
}
