use bytes::Bytes;
use tokio::sync::oneshot::Sender;
use zb_aps::apsde::{ConfirmStatus, DataRequest};

use super::{TransmissionResponse, TransmissionToken};
use crate::Error;

/// Messages exchanged with the APS actor.
#[derive(Debug)]
pub enum Message {
    /// Transmit an APS data frame.
    Transmit {
        /// APS data-service request to submit.
        request: DataRequest<Bytes>,
        /// Channel used to return the deferred APS completion.
        response: Sender<Result<TransmissionResponse, Error>>,
    },

    /// Data confirmation reported by the hardware event stream.
    Confirm {
        /// APS counter of the confirmed transmission.
        counter: u8,
        /// APSDE or propagated NWK completion status.
        status: ConfirmStatus,
    },

    /// Cancel a pending confirmation whose future was dropped.
    Cancel {
        /// Coordinator-private identity of the pending confirmation to cancel.
        token: TransmissionToken,
    },

    /// Expire a pending hardware confirmation.
    ConfirmationTimeout {
        /// Coordinator-private identity whose confirmation timeout elapsed.
        token: TransmissionToken,
    },

    /// Fail every pending acknowledged transmission because the network went down.
    NetworkDown,

    /// Fail every pending transmission and stop because hardware events are unavailable.
    HardwareUnavailable,
}
