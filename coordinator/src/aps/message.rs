use bytes::Bytes;
use tokio::sync::oneshot::Sender;
use zb_aps::Data;
use zb_core::Destination;
use zb_hw::Error;

/// Messages exchanged with the APS actor.
#[derive(Debug)]
pub enum Message {
    /// Transmit an APS data frame.
    Transmit {
        /// Network destination for the frame.
        destination: Destination,
        /// APS data frame to transmit.
        frame: Data<Bytes>,
        /// Optional channel for the completed APS transmission result.
        response: Option<Sender<Result<(), Error>>>,
    },

    /// Result of an acknowledged APS transmission reported by the hardware event stream.
    ApsResponse {
        /// Successful APS counter or hardware failure.
        response: Result<u8, Error>,
    },
}
