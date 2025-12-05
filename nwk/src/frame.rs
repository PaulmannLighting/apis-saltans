use le_stream::ToLeStream;
use zcl::Header;
use zigbee::Cluster;

/// A non-sequenced, non-generic view on a ZCL frame for transmission via channels.
///
/// # Invariants
///
/// The underlying frame's sequence number must be overridden and is assumed to be undefined.
#[derive(Debug)]
pub struct Frame {
    cluster_id: u16,
    header: Header,
    payload: Box<[u8]>,
}

impl Frame {
    /// Set the sequence number of the ZCL frame.
    ///
    /// The resulting frame will have a well-defined sequence number and can be serialized and sent.
    #[must_use]
    pub fn with_seq(mut self, seq: u8) -> SequencedFrame {
        self.header.set_seq(seq);
        SequencedFrame {
            cluster_id: self.cluster_id,
            header: self.header,
            payload: self.payload,
        }
    }
}

impl<T> From<zcl::Frame<T>> for Frame
where
    T: Cluster + ToLeStream,
{
    fn from(frame: zcl::Frame<T>) -> Self {
        let (header, payload) = frame.into_parts();
        Self {
            cluster_id: <T as Cluster>::ID,
            header,
            payload: payload.to_le_stream().collect(),
        }
    }
}

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
    header: Header,
    payload: Box<[u8]>,
}

impl SequencedFrame {
    /// Return the cluster ID of the ZCL frame.
    #[must_use]
    pub const fn cluster_id(&self) -> u16 {
        self.cluster_id
    }

    /// Serialize the ZCL frame into a little-endian byte array.
    #[must_use]
    pub fn serialize(self) -> Box<[u8]> {
        self.header.to_le_stream().chain(self.payload).collect()
    }
}
