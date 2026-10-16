//! State for ZDP requests being submitted to APS.

use std::collections::BTreeMap;
use std::mem::take;

use bytes::Bytes;
use log::{debug, warn};
use tokio::sync::mpsc::WeakSender;
use tokio::sync::oneshot::{Receiver, Sender};
use tokio::task::AbortHandle;
use zb_aps::apsde::DataRequest;
use zb_hw::{Error as HardwareError, TransmissionError};
use zb_zdp::Command;

use super::Message;
use crate::Error;
use crate::aps::{Aps, TransmissionResponse};
use crate::correlation::{Cancellation, CancellationMessage, Registry, Token};
use crate::response::ApsProtocolResponse;
use crate::runtime::spawn;
use crate::zdp::ZdpKey;

const INITIAL_ID: u64 = 0;
const ID_STEP: u64 = 1;

/// Owns outbound APS handoffs, including caller completion and correlation cleanup.
#[derive(Debug)]
pub(super) struct Submissions {
    pending: BTreeMap<u64, CommunicationSubmission>,
    next_id: u64,
    limit: usize,
    aps: Aps,
    inbox: WeakSender<Message>,
}

/// Actor-owned state retained while a ZDP request is being handed to APS.
#[derive(Debug)]
struct CommunicationSubmission {
    token: Token<ZdpKey>,
    protocol_response: Receiver<Result<Command, Error>>,
    response: Sender<Result<ApsProtocolResponse<Command>, Error>>,
    task: AbortHandle,
}

impl CommunicationSubmission {
    /// Deliver an accepted APS submission together with its protocol response.
    fn complete(self, transmission: TransmissionResponse, cancellation: Cancellation) {
        self.response
            .send(Ok(ApsProtocolResponse::new(
                transmission,
                self.protocol_response,
                cancellation,
            )))
            .unwrap_or_else(drop);
    }

    /// Stop the submission task and deliver its terminal error.
    fn fail(self, error: Error) {
        self.task.abort();
        self.response.send(Err(error)).unwrap_or_else(drop);
    }
}

impl Submissions {
    /// Create an empty bounded handoff collection.
    pub(super) const fn new(aps: Aps, inbox: WeakSender<Message>, limit: usize) -> Self {
        Self {
            pending: BTreeMap::new(),
            next_id: INITIAL_ID,
            limit,
            aps,
            inbox,
        }
    }

    /// Change the admission limit without cancelling existing work.
    pub(super) const fn set_limit(&mut self, limit: usize) {
        self.limit = limit;
    }

    /// Check admission before reserving a protocol correlation.
    pub(super) fn has_capacity(&self) -> bool {
        if self.pending.len() < self.limit {
            return true;
        }
        warn!(
            "Rejecting ZDP communication because the submission limit of {} has been reached",
            self.limit
        );
        false
    }

    /// Start an admitted handoff and retain its caller and protocol response until completion.
    pub(super) fn start(
        &mut self,
        request: DataRequest<Bytes>,
        token: Token<ZdpKey>,
        protocol_response: Receiver<Result<Command, Error>>,
        response: Sender<Result<ApsProtocolResponse<Command>, Error>>,
    ) {
        let aps = self.aps.clone();
        let inbox = self.inbox.clone();
        let id = self.allocate_id();
        let task = spawn(async move {
            let result = aps.transmit(request).await;
            let Some(inbox) = inbox.upgrade() else {
                return;
            };
            inbox
                .send(Message::CommunicationSubmissionFinished { id, result })
                .await
                .unwrap_or_else(|error| {
                    debug!("Failed to complete ZDP communication submission: {error}");
                });
        });
        let previous = self.pending.insert(
            id,
            CommunicationSubmission {
                token,
                protocol_response,
                response,
                task: task.abort_handle(),
            },
        );
        debug_assert!(previous.is_none());
    }

    /// Retire a submission and complete its caller, releasing failed correlations.
    pub(super) fn finish(
        &mut self,
        id: u64,
        result: Result<TransmissionResponse, Error>,
        responses: &mut Registry<ZdpKey, Command>,
    ) {
        let Some(submission) = self.pending.remove(&id) else {
            debug!("Ignoring completion for unknown ZDP communication submission {id}");
            return;
        };
        match result {
            Ok(transmission) => {
                let cancellation =
                    Message::cancellation(self.inbox.clone(), submission.token, "ZDP");
                submission.complete(transmission, cancellation);
            }
            Err(error) => {
                responses.discard(submission.token);
                submission.fail(error);
            }
        }
    }

    /// Fail pending handoffs while preserving their exposed sequence numbers in quarantine.
    pub(super) fn network_down(
        &mut self,
        responses: &mut Registry<ZdpKey, Command>,
    ) -> Vec<Token<ZdpKey>> {
        let submissions = take(&mut self.pending);
        let protected = submissions
            .values()
            .map(|submission| submission.token)
            .collect::<Vec<_>>();
        let quarantined = responses.network_down_preserving(
            &TransmissionError::Network(zb_hw::NetworkError::NetworkDown),
            protected,
        );
        for submission in submissions.into_values() {
            submission.fail(
                HardwareError::from(TransmissionError::Network(zb_hw::NetworkError::NetworkDown))
                    .into(),
            );
        }
        quarantined
    }

    /// Stop handoffs and notify their callers that the hardware actor is unavailable.
    pub(super) fn hardware_unavailable(&mut self) {
        for submission in take(&mut self.pending).into_values() {
            submission.fail(HardwareError::ActorUnavailable.into());
        }
    }

    /// Cancel all handoffs and close caller channels when the actor inbox closes.
    pub(super) fn abort(&mut self) {
        for submission in take(&mut self.pending).into_values() {
            submission.task.abort();
        }
    }

    fn allocate_id(&mut self) -> u64 {
        loop {
            let id = self.next_id;
            self.next_id = self.next_id.wrapping_add(ID_STEP);
            if !self.pending.contains_key(&id) {
                return id;
            }
        }
    }
}

impl Drop for Submissions {
    fn drop(&mut self) {
        self.hardware_unavailable();
    }
}
