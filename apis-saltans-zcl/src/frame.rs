//! ZCL frame representation.

use apis_saltans_aps::Data;
use apis_saltans_core::{ClusterSpecific, Profile, Profiled};
use bytes::Bytes;
use le_stream::{FromLeStream, ToLeStream};

pub use self::header::{Control, Direction, Header, Scope};
pub use self::parse_frame_error::ParseFrameError;
use crate::command::Scoped;
use crate::{Command, ParseDirection};

mod header;
mod parse_frame_error;

/// A ZCL frame.
#[derive(Clone, Debug, Eq, Hash, PartialEq, FromLeStream, ToLeStream)]
pub struct Frame<T> {
    header: Header,
    payload: T,
}

impl<T> Frame<T> {
    /// Create a cluster-specific ZCL frame for an outgoing command payload.
    ///
    /// The frame header is derived from the payload type's scope, direction,
    /// default-response behavior, manufacturer code, and command ID.
    #[must_use]
    pub fn cluster_specific(seq: u8, payload: T) -> Self
    where
        T: ClusterSpecific + Command + Scoped,
    {
        let header = Header::new(
            T::SCOPE,
            T::DIRECTION,
            T::DISABLE_DEFAULT_RESPONSE,
            T::MANUFACTURER_CODE,
            seq,
            <T as Command>::ID,
        );
        Self { header, payload }
    }

    /// Create a global ZCL frame for an outgoing command payload.
    ///
    /// The frame header is derived from the payload type's scope, direction,
    /// default-response behavior, and command ID. The supplied manufacturer
    /// code is written into the header when the global command is
    /// manufacturer-specific.
    #[must_use]
    pub fn global(seq: u8, payload: T, manufacturer_code: Option<u16>) -> Self
    where
        T: Command + Scoped,
    {
        let header = Header::new(
            T::SCOPE,
            T::DIRECTION,
            T::DISABLE_DEFAULT_RESPONSE,
            manufacturer_code,
            seq,
            <T as Command>::ID,
        );
        Self { header, payload }
    }

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

impl<T> ClusterSpecific for Frame<T>
where
    T: ClusterSpecific,
{
    const ID: u16 = T::ID;
}

impl<T> Profiled for Frame<T>
where
    T: Profiled,
{
    const PROFILE: Profile = T::PROFILE;
}

impl<T> Command for Frame<T>
where
    T: Command,
{
    const ID: u8 = T::ID;
    const DIRECTION: Direction = T::DIRECTION;
    const PARSE_DIRECTION: ParseDirection = T::PARSE_DIRECTION;
    const MANUFACTURER_CODE: Option<u16> = T::MANUFACTURER_CODE;
    const DISABLE_DEFAULT_RESPONSE: bool = T::DISABLE_DEFAULT_RESPONSE;
}

/// A parsed ZCL frame.
impl Frame<crate::Cluster> {
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
        let payload = crate::Cluster::parse_zcl_cluster(cluster_id, header, bytes)?;
        Ok(Self { header, payload })
    }
}

impl TryFrom<Data<Bytes>> for Frame<crate::Cluster> {
    type Error = ParseFrameError;

    fn try_from(frame: Data<Bytes>) -> Result<Self, Self::Error> {
        let (header, payload) = frame.into_parts();
        Self::parse(header.cluster_id(), payload.into_iter())
    }
}
