use le_stream::ToLeStream;
use zigbee::{ClusterId, Endpoint};

/// A non-sequenced, non-generic view on a ZCL frame for transmission via channels.
///
/// # Invariants
///
/// The underlying frame's sequence number must be overridden and is assumed to be undefined.
#[derive(Debug)]
pub struct Frame {
    cluster_id: u16,
    profile_id: Option<u16>,
    source_endpoint: Option<Endpoint>,
    payload: Box<[u8]>,
}

impl Frame {
    /// Create a new `Frame`.
    #[must_use]
    pub(crate) fn new<T>(
        profile_id: Option<u16>,
        source_endpoint: Option<Endpoint>,
        payload: T,
    ) -> Self
    where
        T: ClusterId + ToLeStream,
    {
        Self {
            cluster_id: payload.cluster_id(),
            profile_id,
            source_endpoint,
            payload: payload.to_le_stream().collect(),
        }
    }

    /// Return the cluster ID and payload of the frame.
    #[must_use]
    pub fn into_parts(self) -> (u16, Option<u16>, Option<Endpoint>, Box<[u8]>) {
        (
            self.cluster_id,
            self.profile_id,
            self.source_endpoint,
            self.payload,
        )
    }
}

impl<T> From<zcl::Frame<T>> for Frame
where
    T: ClusterId + ToLeStream,
{
    fn from(frame: zcl::Frame<T>) -> Self {
        Self::new(None, None, frame)
    }
}

impl<T> From<zdp::Frame<T>> for Frame
where
    T: ClusterId + ToLeStream,
{
    fn from(frame: zdp::Frame<T>) -> Self {
        Self::new(Some(0x0000), Some(Endpoint::Data), frame)
    }
}
