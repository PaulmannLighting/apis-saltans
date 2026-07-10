//! General-purpose APS frame.

use bytes::Bytes;
use le_stream::ToLeStream;
use zb_core::{ClusterSpecific, Profile};
use zb_hw::Metadata;

/// A simplified APS frame.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Payload {
    /// APS metadata for transmission.
    metadata: Metadata,
    /// Command payload.
    payload: Bytes,
}

impl Payload {
    /// Consume the frame into its parts.
    #[must_use]
    pub fn into_parts(self) -> (Metadata, Bytes) {
        (self.metadata, self.payload)
    }
}

impl<T> From<T> for Payload
where
    T: ClusterSpecific + ToLeStream,
{
    fn from(zcl_command: T) -> Self {
        Self {
            metadata: Metadata::new(Profile::Network, T::ID),
            payload: zcl_command.to_le_stream().collect(),
        }
    }
}
