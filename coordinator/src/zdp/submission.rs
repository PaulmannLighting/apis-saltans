//! State for ZDP requests being submitted to APS.

use tokio::sync::oneshot::{Receiver, Sender};
use tokio::task::AbortHandle;
use zb_zdp::Command;

use crate::Error;
use crate::aps::TransmissionResponse;
use crate::correlation::{Cancellation, Token};
use crate::response::ApsProtocolResponse;

/// Actor-owned state retained while a ZDP request is being handed to APS.
#[derive(Debug)]
pub(super) struct CommunicationSubmission {
    pub(super) token: Token,
    pub(super) protocol_response: Receiver<Result<Command, Error>>,
    pub(super) response: Sender<Result<ApsProtocolResponse<Command>, Error>>,
    pub(super) task: AbortHandle,
}

impl CommunicationSubmission {
    /// Deliver an accepted APS submission together with its protocol response.
    pub(super) fn complete(self, transmission: TransmissionResponse, cancellation: Cancellation) {
        self.response
            .send(Ok(ApsProtocolResponse::new(
                transmission,
                self.protocol_response,
                cancellation,
            )))
            .unwrap_or_else(drop);
    }

    /// Stop the submission task and deliver its terminal error.
    pub(super) fn fail(self, error: Error) {
        self.task.abort();
        self.response.send(Err(error)).unwrap_or_else(drop);
    }
}
