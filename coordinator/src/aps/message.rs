use bytes::Bytes;
use tokio::sync::oneshot::Sender;
use zb_core::Destination;
use zb_hw::Error;

use super::Metadata;

/// Messages exchanged with the APS actor.
#[derive(Debug)]
pub enum Message {
    /// Transmit an APS data frame.
    Transmit {
        /// Network destination for the frame.
        destination: Destination,
        /// Metadata used by the APS actor to construct the frame header.
        metadata: Metadata,
        /// Serialized application payload.
        payload: Bytes,
        /// Optional channel for the completed APS transmission result.
        response: Option<Sender<Result<(), Error>>>,
    },

    /// Successful acknowledgement reported by the hardware event stream.
    Ack {
        /// APS counter of the acknowledged transmission.
        sequence: u8,
    },

    /// Failed acknowledgement reported by the hardware event stream.
    Nak {
        /// APS counter of the rejected transmission.
        sequence: u8,
        /// Hardware failure reported for the transmission.
        error: Error,
    },
}
