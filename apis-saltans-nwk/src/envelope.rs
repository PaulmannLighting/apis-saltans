use crate::{Metadata, Sender};

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
    #[must_use]
    pub const fn new(source: Sender, metadata: Metadata, payload: T) -> Self {
        Self {
            source,
            metadata,
            payload,
        }
    }

    #[must_use]
    pub const fn source(&self) -> Sender {
        self.source
    }

    #[must_use]
    pub const fn metadata(&self) -> Metadata {
        self.metadata
    }

    #[must_use]
    pub const fn payload(&self) -> &T {
        &self.payload
    }

    #[must_use]
    pub fn into_parts(self) -> (Sender, Metadata, T) {
        (self.source, self.metadata, self.payload)
    }
}
