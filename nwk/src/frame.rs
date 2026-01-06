use le_stream::ToLeStream;
use zigbee::{ClusterId, Endpoint, Profile};

pub use self::aps_metadata::ApsMetadata;

mod aps_metadata;

/// A non-sequenced, non-generic view on a ZCL frame for transmission via channels.
///
/// # Invariants
///
/// The underlying frame's sequence number must be overridden and is assumed to be undefined.
#[derive(Debug)]
pub struct Frame {
    aps_metadata: ApsMetadata,
    payload: Box<[u8]>,
}

impl Frame {
    /// Create a new `Frame`.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the `aps_metadata` and `payload` are valid and consistent with each other.
    #[expect(unsafe_code)]
    #[must_use]
    pub const unsafe fn new(aps_metadata: ApsMetadata, payload: Box<[u8]>) -> Self {
        Self {
            aps_metadata,
            payload,
        }
    }

    /// Return the cluster ID and payload of the frame.
    #[must_use]
    pub fn into_parts(self) -> (ApsMetadata, Box<[u8]>) {
        (self.aps_metadata, self.payload)
    }
}

impl<T> From<zcl::Frame<T>> for Frame
where
    T: ClusterId + ToLeStream,
{
    fn from(frame: zcl::Frame<T>) -> Self {
        #[expect(unsafe_code)]
        // SAFETY: We ensure that the ApsMetadata contains the correct cluster ID.
        unsafe {
            Self::new(
                ApsMetadata::new(frame.cluster_id(), None, None),
                frame.to_le_stream().collect(),
            )
        }
    }
}

impl<T> From<zdp::Frame<T>> for Frame
where
    T: ClusterId + ToLeStream,
{
    fn from(frame: zdp::Frame<T>) -> Self {
        #[expect(unsafe_code)]
        // SAFETY: We ensure that the ApsMetadata contains the correct cluster ID, profile ID and endpoint.
        unsafe {
            Self::new(
                ApsMetadata::new(
                    frame.cluster_id(),
                    Some(Profile::Network),
                    Some(Endpoint::Data),
                ),
                frame.to_le_stream().collect(),
            )
        }
    }
}
