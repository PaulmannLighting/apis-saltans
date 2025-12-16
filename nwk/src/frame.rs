use le_stream::ToLeStream;
use zigbee::ClusterId;

/// A non-sequenced, non-generic view on a ZCL frame for transmission via channels.
///
/// # Invariants
///
/// The underlying frame's sequence number must be overridden and is assumed to be undefined.
#[derive(Debug)]
pub struct Frame {
    cluster_id: u16,
    payload: Box<[u8]>,
}

impl Frame {
    /// Create a new `Frame`.
    #[must_use]
    pub(crate) fn new<T>(payload: T) -> Self
    where
        T: ClusterId + ToLeStream,
    {
        Self {
            cluster_id: payload.cluster_id(),
            payload: payload.to_le_stream().collect(),
        }
    }

    /// Return the cluster ID and payload of the frame.
    #[must_use]
    pub fn into_parts(self) -> (u16, Box<[u8]>) {
        (self.cluster_id, self.payload)
    }
}

impl<T> From<zcl::Frame<T>> for Frame
where
    T: ClusterId + ToLeStream,
{
    fn from(frame: zcl::Frame<T>) -> Self {
        Self::new(frame)
    }
}

impl<T> From<zdp::Frame<T>> for Frame
where
    T: ClusterId + ToLeStream,
{
    fn from(frame: zdp::Frame<T>) -> Self {
        Self::new(frame)
    }
}
