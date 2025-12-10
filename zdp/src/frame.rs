use le_stream::{FromLeStream, ToLeStream};
use zigbee::Cluster;

use crate::{Command, ParseFrameError, Service};

/// A frame with a sequence number and associated data.
#[derive(Clone, Debug, Eq, PartialEq, Hash, ToLeStream, FromLeStream)]
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
}

impl<T> Frame<T>
where
    T: Service,
{
    /// Returns the service name.
    #[must_use]
    pub const fn service_name(&self) -> &'static str {
        T::NAME
    }
}

impl<T> Frame<T>
where
    T: Cluster,
{
    /// Returns the cluster ID.
    #[must_use]
    pub const fn cluster_id(&self) -> u16 {
        <T as Cluster>::ID
    }
}

impl Frame<Command> {
    /// Parses a `Frame<Command>` from the given cluster ID and byte iterator.
    ///
    /// # Errors
    ///
    /// Returns [`ParseFrameError`] if parsing fails.
    pub fn parse<T>(cluster_id: u16, mut bytes: T) -> Result<Self, ParseFrameError>
    where
        T: Iterator<Item = u8>,
    {
        let seq = u8::from_le_stream(&mut bytes).ok_or(ParseFrameError::MissingSeq)?;
        let data = Command::parse(cluster_id, bytes)?;
        Ok(Self { seq, data })
    }
}
