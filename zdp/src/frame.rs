use le_stream::{FromLeStream, ToLeStream};
use zigbee::Cluster;

use crate::{Command, Service};

/// A frame with a sequence number and associated data.
#[derive(Clone, Debug, Eq, PartialEq, Hash, ToLeStream)]
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

impl<T> Cluster for Frame<T>
where
    T: Cluster,
{
    const ID: u16 = T::ID;
}

impl<T> Service for Frame<T>
where
    T: Service,
{
    const NAME: &'static str = T::NAME;
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
