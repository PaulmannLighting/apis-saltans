//! State for ZDP requests being submitted to APS.

use tokio::sync::oneshot::{Receiver, Sender};
use tokio::task::AbortHandle;
use zb_zdp::Command;

use crate::correlation::Token;
use crate::response::ApsProtocolResponse;

/// Actor-owned state retained while a ZDP request is being handed to APS.
#[derive(Debug)]
pub(super) struct CommunicationSubmission {
    pub(super) token: Token,
    pub(super) protocol_response: Receiver<Result<Command, crate::Error>>,
    pub(super) response: Sender<Result<ApsProtocolResponse<Command>, crate::Error>>,
    pub(super) task: AbortHandle,
}
