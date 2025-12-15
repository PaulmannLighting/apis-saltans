use le_stream::ToLeStream;
use zigbee::Cluster;

pub use self::header::Header;
use self::sequenced::SequencedFrame;

mod header;
mod sequenced;

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
    /// Create a new `Frame`.
    #[must_use]
    pub(crate) fn new<T>(header: Header, payload: T) -> Self
    where
        T: Cluster + ToLeStream,
    {
        Self {
            cluster_id: <T as Cluster>::ID,
            header,
            payload: payload.to_le_stream().collect(),
        }
    }

    /// Set the sequence number of the ZCL frame.
    ///
    /// The resulting frame will have a well-defined sequence number and can be serialized and sent.
    #[must_use]
    pub fn with_seq(mut self, seq: u8) -> SequencedFrame {
        match self.header {
            Header::Zcl(ref mut header) => {
                header.set_seq(seq);
            }
            Header::Zdp(ref mut transaction_seq) => {
                *transaction_seq = seq;
            }
        }
        SequencedFrame::new(self.cluster_id, self.header, self.payload)
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
            header: Header::Zcl(header),
            payload: payload.to_le_stream().collect(),
        }
    }
}

impl<T> From<zdp::Frame<T>> for Frame
where
    T: Cluster + ToLeStream,
{
    fn from(frame: zdp::Frame<T>) -> Self {
        let (seq, payload) = frame.into_parts();
        Self {
            cluster_id: <T as Cluster>::ID,
            header: Header::Zdp(seq),
            payload: payload.to_le_stream().collect(),
        }
    }
}
