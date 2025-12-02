use le_stream::{FromLeStream, ToLeStream};

use crate::Service;

/// A frame with a sequence number and associated data.
#[derive(Clone, Debug, Eq, PartialEq, ToLeStream, FromLeStream)]
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

    /// Returns the cluster ID.
    #[must_use]
    pub const fn cluster_id(&self) -> u16 {
        T::CLUSTER_ID
    }
}
