//! Transmission frame abstraction for the network layer.
//!
//! This module provides [`Frame`] and [`Metadata`] types that abstract over
//! application-layer protocols (ZCL, ZDP) for transmission through the network layer.
//! Unlike raw APS frames, these types hide implementation details like frame counters
//! and extended headers, providing a clean interface for coordinator implementations.

use le_stream::ToLeStream;
use zigbee::{ClusterId, Endpoint, Profile};

pub use self::metadata::Metadata;

mod metadata;

/// A transmission-ready frame with application-layer metadata.
///
/// This frame type abstracts over ZCL and ZDP frames, providing a unified
/// representation for transmission via the network layer. It contains:
/// - A serialized payload (ZCL or ZDP frame)
/// - Metadata: cluster ID, profile ID (optional), source endpoint (optional)
///
/// # Invariants
///
/// The underlying frame's sequence number must be overridden and is assumed to be undefined.
#[derive(Debug)]
pub struct Frame {
    aps_metadata: Metadata,
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
    pub const unsafe fn new(aps_metadata: Metadata, payload: Box<[u8]>) -> Self {
        Self {
            aps_metadata,
            payload,
        }
    }

    /// Return the metadata.
    #[must_use]
    pub const fn metadata(&self) -> &Metadata {
        &self.aps_metadata
    }

    /// Return the metadata.
    #[must_use]
    pub const fn metadata_mut(&mut self) -> &mut Metadata {
        &mut self.aps_metadata
    }

    /// Return the cluster ID and payload of the frame.
    #[must_use]
    pub fn into_parts(self) -> (Metadata, Box<[u8]>) {
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
                Metadata::new(frame.cluster_id(), None, None),
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
                Metadata::new(
                    frame.cluster_id(),
                    Some(Profile::Network),
                    Some(Endpoint::Data),
                ),
                frame.to_le_stream().collect(),
            )
        }
    }
}
