use le_stream::{FromLeStream, FromLeStreamTagged, ToLeStream};
use zigbee::Cluster;

use crate::Service;

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

impl<T> FromLeStreamTagged for Frame<T>
where
    T: FromLeStreamTagged<Tag = u16>,
{
    type Tag = u16;

    fn from_le_stream_tagged<I>(
        cluster_id: Self::Tag,
        mut bytes: I,
    ) -> Result<Option<Self>, Self::Tag>
    where
        I: Iterator<Item = u8>,
    {
        let Some(seq) = u8::from_le_stream(&mut bytes) else {
            return Ok(None);
        };

        T::from_le_stream_tagged(cluster_id, bytes)
            .map(|data| data.map(|data| Self::new(seq, data)))
    }
}
