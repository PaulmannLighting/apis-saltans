//! Actor for transmitting APS data frames.

use std::collections::BTreeMap;
use std::time::Duration;

use bytes::Bytes;
use log::warn;
use tokio::spawn;
use tokio::sync::mpsc::{Receiver, Sender, WeakSender};
use tokio::sync::oneshot::{Sender as OneshotSender, channel};
use tokio::time::sleep;
use zb_aps::TxOptions;
use zb_aps::apsde::{ConfirmStatus, DataRequest, IndividualEndpoint, RequestDestination};
use zb_hw::NcpHandle;

pub use self::message::Message;
pub use self::metadata::Metadata;
pub use self::transmission_response::TransmissionResponse;
use crate::MPSC_CHANNEL_SIZE;

mod message;
mod metadata;
mod transmission_response;

const CONFIRMATION_TIMEOUT: Duration = Duration::from_secs(30);
const APS_COUNTER_COUNT: usize = 1_usize << u8::BITS;
const INITIAL_COUNTER: u8 = 0;
const INITIAL_GENERATION: u64 = 0;

type PendingResponse = tokio::sync::oneshot::Sender<Result<(), zb_hw::Error>>;

/// Coordinator-private identity for lifecycle messages associated with one counter allocation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TransmissionToken {
    counter: u8,
    generation: u64,
}

/// Construct an APS data-service request from coordinator destination metadata.
pub const fn data_request(
    destination: RequestDestination,
    source_endpoint: IndividualEndpoint,
    metadata: Metadata,
    asdu: Bytes,
) -> DataRequest<Bytes> {
    DataRequest::new(
        destination,
        metadata.profile().as_u16(),
        metadata.cluster_id(),
        source_endpoint,
        asdu,
    )
    .with_tx_options(metadata.tx_options())
}

const fn acknowledged<T>(request: &DataRequest<T>) -> bool {
    request
        .tx_options()
        .contains(TxOptions::ACKNOWLEDGED_TRANSMISSION)
        && matches!(
            request.destination(),
            RequestDestination::Network { .. } | RequestDestination::Extended { .. }
        )
}

/// Handle for sending commands to the APS actor.
#[derive(Clone, Debug)]
pub struct Aps(Sender<Message>);

impl Aps {
    /// Wrap an APS actor sender.
    #[must_use]
    pub const fn new(sender: Sender<Message>) -> Self {
        Self(sender)
    }

    /// Queue an APS frame and return its deferred transmission result.
    ///
    /// The returned response first waits for backend acceptance. For an acknowledged unicast, it
    /// then waits for the corresponding hardware completion event.
    pub async fn transmit(
        &self,
        request: DataRequest<Bytes>,
    ) -> Result<TransmissionResponse, crate::Error> {
        let (response, result) = channel();

        self.0
            .send(Message::Transmit { request, response })
            .await
            .map_err(|_| crate::Error::from(zb_hw::Error::ActorUnavailable))?;

        result
            .await
            .map_err(|_| crate::Error::from(zb_hw::Error::ActorUnavailable))?
    }

    /// Forward a hardware APS data confirmation to the APS actor.
    pub async fn confirm(&self, counter: u8, status: ConfirmStatus) -> Result<(), zb_hw::Error> {
        self.0
            .send(Message::Confirm { counter, status })
            .await
            .map_err(|_| zb_hw::Error::ActorUnavailable)
    }

    /// Notify the APS actor that the Zigbee network is down.
    pub async fn network_down(&self) -> Result<(), zb_hw::Error> {
        self.0
            .send(Message::NetworkDown)
            .await
            .map_err(|_| zb_hw::Error::ActorUnavailable)
    }

    /// Notify the APS actor that its hardware event source has terminated.
    pub async fn hardware_unavailable(&self) -> Result<(), zb_hw::Error> {
        self.0
            .send(Message::HardwareUnavailable)
            .await
            .map_err(|_| zb_hw::Error::ActorUnavailable)
    }
}

/// APS transmission actor.
#[derive(Debug)]
pub struct Transceiver {
    ncp: NcpHandle,
    state: TransmissionState,
    inbox: WeakSender<Message>,
}

#[derive(Debug)]
struct TransmissionState {
    next_counter: u8,
    next_generation: u64,
    responses: BTreeMap<u8, PendingTransmission>,
    quarantined: BTreeMap<u8, u64>,
}

#[derive(Debug)]
struct PendingTransmission {
    generation: u64,
    acknowledged: bool,
    phase: TransmissionPhase,
    response: Option<PendingResponse>,
}

#[derive(Clone, Copy, Debug)]
enum TransmissionPhase {
    Submitting { confirmation: Option<ConfirmStatus> },
    AwaitingConfirmation,
}

impl PendingTransmission {
    const fn submitting(
        token: TransmissionToken,
        acknowledged: bool,
        response: PendingResponse,
    ) -> Self {
        Self {
            generation: token.generation,
            acknowledged,
            phase: TransmissionPhase::Submitting { confirmation: None },
            response: Some(response),
        }
    }

    #[cfg(test)]
    const fn awaiting_confirmation(token: TransmissionToken, response: PendingResponse) -> Self {
        Self {
            generation: token.generation,
            acknowledged: true,
            phase: TransmissionPhase::AwaitingConfirmation,
            response: Some(response),
        }
    }

    fn complete(&mut self, result: Result<(), zb_hw::Error>) {
        if let Some(response) = self.response.take() {
            response.send(result).unwrap_or_else(drop);
        }
    }

    fn complete_confirmation(&mut self, status: ConfirmStatus) {
        let result = if status.is_success() {
            Ok(())
        } else {
            Err(zb_hw::TransmissionError::Confirmation(status).into())
        };
        self.complete(result);
    }

    fn fail(&mut self, error: &zb_hw::Error) {
        self.complete(Err(error.clone()));
    }
}

impl TransmissionState {
    const fn new() -> Self {
        Self {
            next_counter: INITIAL_COUNTER,
            next_generation: INITIAL_GENERATION,
            responses: BTreeMap::new(),
            quarantined: BTreeMap::new(),
        }
    }

    /// Allocate a counter that cannot be confused with a pending or late confirmation.
    fn allocate(&mut self) -> Option<TransmissionToken> {
        for _ in 0..APS_COUNTER_COUNT {
            let counter = self.next_counter;
            self.next_counter = self.next_counter.wrapping_add(1);
            if !self.responses.contains_key(&counter) && !self.quarantined.contains_key(&counter) {
                let token = TransmissionToken {
                    counter,
                    generation: self.next_generation,
                };
                self.next_generation = self.next_generation.wrapping_add(1);
                return Some(token);
            }
        }

        None
    }

    fn handle_confirm(&mut self, counter: u8, status: ConfirmStatus) {
        let Some(pending) = self.responses.get_mut(&counter) else {
            if self.quarantined.remove(&counter).is_some() {
                log::debug!("Released quarantined APS counter after late confirmation: {counter}");
                return;
            }
            warn!("Received APS data confirmation for unknown counter: {counter}");
            return;
        };

        match &mut pending.phase {
            TransmissionPhase::Submitting { confirmation } if pending.acknowledged => {
                if confirmation.replace(status).is_some() {
                    warn!("Received duplicate APS data confirmation for counter: {counter}");
                }
                return;
            }
            TransmissionPhase::Submitting { .. } => {
                warn!(
                    "Received APS data confirmation for unacknowledged transmission counter: \
                     {counter}"
                );
                return;
            }
            TransmissionPhase::AwaitingConfirmation => {}
        }

        let mut pending = self
            .responses
            .remove(&counter)
            .expect("pending confirmation remains present");
        pending.complete_confirmation(status);
    }

    /// Store a backend submission under its allocated Zigbee APS counter.
    fn store_submission(
        &mut self,
        token: TransmissionToken,
        acknowledged: bool,
        response: PendingResponse,
    ) {
        let previous = self.responses.insert(
            token.counter,
            PendingTransmission::submitting(token, acknowledged, response),
        );
        debug_assert!(previous.is_none());
        debug_assert!(!self.quarantined.contains_key(&token.counter));
    }

    /// Store an accepted acknowledged response under its allocated Zigbee APS counter.
    #[cfg(test)]
    fn store_pending_response(&mut self, token: TransmissionToken, response: PendingResponse) {
        let previous = self.responses.insert(
            token.counter,
            PendingTransmission::awaiting_confirmation(token, response),
        );
        debug_assert!(previous.is_none());
        debug_assert!(!self.quarantined.contains_key(&token.counter));
    }

    /// Complete a backend submission and report whether to schedule its confirmation deadline.
    fn finish_submission(
        &mut self,
        token: TransmissionToken,
        result: Result<(), zb_hw::Error>,
    ) -> bool {
        if !self.pending_generation_matches(token) {
            return false;
        }
        let mut pending = self
            .responses
            .remove(&token.counter)
            .expect("matching pending generation remains present");

        if let Err(error) = result {
            pending.complete(Err(error));
            return false;
        }
        if !pending.acknowledged {
            pending.complete(Ok(()));
            return false;
        }

        let TransmissionPhase::Submitting { confirmation } = pending.phase else {
            warn!(
                "Received duplicate APS backend submission completion for counter: {}",
                token.counter
            );
            self.responses.insert(token.counter, pending);
            return false;
        };
        if let Some(status) = confirmation {
            pending.complete_confirmation(status);
            return false;
        }

        if pending.response.is_none() {
            self.quarantined.insert(token.counter, token.generation);
        } else {
            pending.phase = TransmissionPhase::AwaitingConfirmation;
            self.responses.insert(token.counter, pending);
        }
        true
    }

    fn cancel(&mut self, token: TransmissionToken) {
        if !self.pending_generation_matches(token) {
            return;
        }
        let submitting = matches!(
            self.responses
                .get(&token.counter)
                .expect("matching pending generation remains present")
                .phase,
            TransmissionPhase::Submitting { .. }
        );
        if submitting {
            self.responses
                .get_mut(&token.counter)
                .expect("matching pending generation remains present")
                .response = None;
        } else {
            self.responses.remove(&token.counter);
            self.quarantined.insert(token.counter, token.generation);
        }
    }

    /// Expire an accepted transmission and report whether its confirmation is still missing.
    fn timeout(&mut self, token: TransmissionToken) -> bool {
        if self.pending_generation_matches(token) {
            let mut pending = self
                .responses
                .remove(&token.counter)
                .expect("matching pending generation remains present");
            self.quarantined.insert(token.counter, token.generation);
            pending.complete(Err(zb_hw::TransmissionError::Timeout.into()));
        }

        self.quarantined.get(&token.counter) == Some(&token.generation)
    }

    fn pending_generation_matches(&self, token: TransmissionToken) -> bool {
        self.responses
            .get(&token.counter)
            .is_some_and(|pending| pending.generation == token.generation)
    }

    fn network_down(&mut self, error: &zb_hw::Error) {
        let responses = std::mem::take(&mut self.responses);
        for (counter, mut pending) in responses {
            pending.fail(error);
            if matches!(pending.phase, TransmissionPhase::Submitting { .. }) {
                self.responses.insert(counter, pending);
            } else {
                self.quarantined.insert(counter, pending.generation);
            }
        }
    }

    fn stop(&mut self, error: &zb_hw::Error) {
        for mut pending in std::mem::take(&mut self.responses).into_values() {
            pending.fail(error);
        }
    }
}

impl Transceiver {
    /// Create an APS actor with its Zigbee APS counter allocator initialized to zero.
    #[must_use]
    pub const fn new(ncp: NcpHandle, inbox: WeakSender<Message>) -> Self {
        Self {
            ncp,
            state: TransmissionState::new(),
            inbox,
        }
    }

    /// Run the APS actor.
    pub async fn run(mut self, mut messages: Receiver<Message>) {
        while let Some(message) = messages.recv().await {
            if !self.handle_actor_message(message) {
                break;
            }
        }
    }

    fn handle_actor_message(&mut self, message: Message) -> bool {
        match message {
            Message::Transmit { request, response } => {
                self.transmit(request, response);
            }
            Message::SubmissionFinished { token, result } => {
                if self.state.finish_submission(token, result) {
                    self.schedule_confirmation_timeout(token);
                }
            }
            Message::Confirm { counter, status } => {
                self.state.handle_confirm(counter, status);
            }
            Message::NetworkDown => {
                self.state
                    .network_down(&zb_hw::TransmissionError::NoRoute.into());
            }
            Message::HardwareUnavailable => {
                self.state.stop(&zb_hw::Error::ActorUnavailable);
                return false;
            }
            Message::Cancel { token } => {
                self.state.cancel(token);
            }
            Message::ConfirmationTimeout { token } => {
                if self.state.timeout(token) {
                    warn!(
                        "APS confirmation for counter {} did not arrive before its deadline; \
                         stopping the APS actor to prevent unsafe counter reuse",
                        token.counter
                    );
                    return false;
                }
            }
        }
        true
    }

    /// Assign an APS counter and submit a data-service request to the hardware actor.
    fn transmit(
        &mut self,
        request: DataRequest<Bytes>,
        response: OneshotSender<Result<TransmissionResponse, crate::Error>>,
    ) {
        let acknowledged = acknowledged(&request);
        let Some(token) = self.state.allocate() else {
            response
                .send(Err(crate::Error::ApsCounterExhausted))
                .unwrap_or_else(drop);
            return;
        };
        let (completion, result) = channel();
        let transmission = TransmissionResponse::new(result, token, self.inbox.clone());
        if let Err(transmission) = response.send(Ok(transmission)) {
            drop(transmission);
            return;
        }

        self.state.store_submission(token, acknowledged, completion);
        self.spawn_transmission(request, token);
    }

    fn spawn_transmission(&self, request: DataRequest<Bytes>, token: TransmissionToken) {
        let ncp = self.ncp.clone();
        let inbox = self.inbox.clone();
        spawn(async move {
            let result = ncp.transmit(request, token.counter).await;
            let Some(inbox) = inbox.upgrade() else {
                return;
            };
            inbox
                .send(Message::SubmissionFinished { token, result })
                .await
                .unwrap_or_else(|error| {
                    log::debug!("Failed to enqueue APS backend submission completion: {error}");
                });
        });
    }

    fn schedule_confirmation_timeout(&self, token: TransmissionToken) {
        let inbox = self.inbox.clone();
        spawn(async move {
            sleep(CONFIRMATION_TIMEOUT).await;
            let Some(inbox) = inbox.upgrade() else {
                return;
            };
            inbox
                .send(Message::ConfirmationTimeout { token })
                .await
                .unwrap_or_else(|error| {
                    log::debug!("Failed to enqueue APS confirmation timeout: {error}");
                });
        });
    }

    /// Spawn the APS actor.
    pub fn spawn(ncp: NcpHandle) -> Aps {
        let (aps_tx, aps_rx) = tokio::sync::mpsc::channel(MPSC_CHANNEL_SIZE);
        spawn(Self::new(ncp, aps_tx.downgrade()).run(aps_rx));
        Aps::new(aps_tx)
    }
}

#[cfg(test)]
mod tests;
