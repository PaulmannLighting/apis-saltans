use bytes::Bytes;
use tokio::sync::oneshot::Sender;
use zb_aps::apsde::{ConfirmStatus, DataRequest};
use zb_hw::Error;

/// Messages exchanged with the APS actor.
#[derive(Debug)]
pub enum Message {
    /// Transmit an APS data frame.
    Transmit {
        /// APS data-service request to submit.
        request: DataRequest<Bytes>,
        /// Channel for backend acceptance and, when requested, APS completion.
        response: Sender<Result<(), Error>>,
    },

    /// Data confirmation reported by the hardware event stream.
    Confirm {
        /// APS counter of the confirmed transmission.
        counter: u8,
        /// APSDE or propagated NWK completion status.
        status: ConfirmStatus,
    },
}
