use std::fmt::Display;

use bytes::Bytes;
use le_stream::{FromLeStream, ToLeStream};
use zb_aps::{Data, Destination};
use zb_core::Endpoint;

pub use self::parse_frame_error::ParseFrameError;
use crate::Command;

mod parse_frame_error;

/// A frame with a sequence number and associated data.
#[derive(Clone, Debug, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
pub struct Frame<T> {
    seq: u8,
    data: T,
}

impl<T> Frame<T> {
    /// Creates a new `Frame` with the given sequence number and data.
    #[must_use]
    pub const fn new(seq: u8, data: T) -> Self {
        Self { seq, data }
    }

    /// Returns the sequence number.
    #[must_use]
    pub const fn seq(&self) -> u8 {
        self.seq
    }

    /// Returns a reference to the associated data.
    #[must_use]
    pub const fn data(&self) -> &T {
        &self.data
    }

    /// Decomposes the frame into its sequence number and associated data.
    #[must_use]
    pub fn into_parts(self) -> (u8, T) {
        (self.seq, self.data)
    }
}

impl Frame<Command> {
    /// Parses a `Frame` from a byte stream with the given cluster ID.
    ///
    /// # Errors
    ///
    /// Returns an error if parsing the command fails.
    pub fn parse_with_cluster_id<T>(cluster_id: u16, mut bytes: T) -> Result<Option<Self>, u16>
    where
        T: Iterator<Item = u8>,
    {
        let Some(seq) = u8::from_le_stream(&mut bytes) else {
            return Ok(None);
        };

        Command::parse_with_cluster_id(cluster_id, bytes)
            .map(|data| data.map(|data| Self::new(seq, data)))
    }
}

impl<T> Display for Frame<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Frame {{ seq: {:#04X}, data: {} }}", self.seq, self.data)
    }
}

impl TryFrom<Data<Bytes>> for Frame<Command> {
    type Error = ParseFrameError;

    fn try_from(frame: Data<Bytes>) -> Result<Self, Self::Error> {
        let (header, payload) = frame.into_parts();

        if !matches!(
            header.destination(),
            Destination::Unicast(endpoint) | Destination::Broadcast(endpoint)
                if endpoint == Endpoint::Data
        ) {
            return Err(Self::Error::Destination(header.destination()));
        }

        Self::parse_with_cluster_id(header.cluster_id(), payload.into_iter())
            .map_err(Self::Error::ClusterId)?
            .ok_or(Self::Error::Invalid)
    }
}
