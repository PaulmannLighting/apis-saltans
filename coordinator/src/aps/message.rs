use bytes::Bytes;
use tokio::sync::oneshot::Sender;
use zb_core::{Destination, Endpoint};
use zb_hw::{Error, TransmissionError};

use super::Metadata;

/// Messages exchanged with the APS actor.
#[derive(Debug)]
pub enum Message {
    /// Transmit an APS data frame.
    Transmit {
        /// Network destination for the frame.
        destination: Destination,
        /// Local source endpoint of the frame.
        source_endpoint: Endpoint,
        /// Metadata used by the APS actor to construct the frame header.
        metadata: Metadata,
        /// Serialized application payload.
        payload: Bytes,
        /// Channel for backend acceptance and, when requested, APS completion.
        response: Sender<Result<(), Error>>,
    },

    /// Successful transmission reported by the hardware event stream.
    Ack {
        /// APS counter of the acknowledged transmission.
        counter: u8,
    },

    /// Failed transmission reported by the hardware event stream.
    Nak {
        /// APS counter of the failed transmission.
        counter: u8,
        /// Hardware failure reported for the transmission.
        error: TransmissionError,
    },
}
