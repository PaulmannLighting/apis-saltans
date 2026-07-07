use crate::{Metadata, Sender};

/// A payload together with its network-layer source and metadata.
///
/// `Envelope` is generic over the carried payload so higher layers can attach
/// NWK context without tying this crate to APS, ZCL, ZDP, or hardware-specific
/// frame types.
#[cfg_attr(
    feature = "le-stream",
    derive(le_stream::FromLeStream, le_stream::ToLeStream)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Envelope<T> {
    source: Sender,
    metadata: Metadata,
    payload: T,
}

impl<T> Envelope<T> {
    /// Create a new envelope.
    #[must_use]
    pub const fn new(source: Sender, metadata: Metadata, payload: T) -> Self {
        Self {
            source,
            metadata,
            payload,
        }
    }

    /// Return the network-layer sender.
    #[must_use]
    pub const fn source(&self) -> Sender {
        self.source
    }

    /// Return the network-layer metadata.
    #[must_use]
    pub const fn metadata(&self) -> Metadata {
        self.metadata
    }

    /// Return the enclosed payload by reference.
    #[must_use]
    pub const fn payload(&self) -> &T {
        &self.payload
    }

    /// Split the envelope into sender, metadata, and payload.
    #[must_use]
    pub fn into_parts(self) -> (Sender, Metadata, T) {
        (self.source, self.metadata, self.payload)
    }
}
