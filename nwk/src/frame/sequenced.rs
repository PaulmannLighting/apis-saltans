use le_stream::ToLeStream;

use super::typ::Type;

/// A sequenced, non-generic view on a ZCL frame for transmission via channels.
///
/// # Invariants
///
/// This frame is guaranteed to have a well-defined sequence.
/// It can be safely serialized and sent.
///
/// The only way to create this frame is via [`Frame::with_seq`].
#[derive(Debug)]
pub struct SequencedFrame {
    cluster_id: u16,
    typ: Type,
    payload: Box<[u8]>,
}

impl SequencedFrame {
    /// Creates a new `SequencedFrame`.
    #[must_use]
    pub(crate) const fn new(cluster_id: u16, typ: Type, payload: Box<[u8]>) -> Self {
        Self {
            cluster_id,
            typ,
            payload,
        }
    }

    /// Return the cluster ID of the ZCL frame.
    #[must_use]
    pub const fn cluster_id(&self) -> u16 {
        self.cluster_id
    }

    /// Serialize the ZCL frame into a little-endian byte array.
    #[must_use]
    pub fn serialize(self) -> Box<[u8]> {
        self.typ.to_le_stream().chain(self.payload).collect()
    }
}
