//! Actor for transmitting APS data frames.

use std::collections::{BTreeMap, BTreeSet};
use std::time::Duration;

use bytes::Bytes;
use log::warn;
use tokio::spawn;
use tokio::sync::mpsc::{Receiver, Sender, WeakSender};
use tokio::sync::oneshot::{Sender as OneshotSender, channel};
use tokio::time::sleep;
use zb_aps::TxOptions;
use zb_aps::apsde::{
    BroadcastAddress, ConfirmStatus, DataRequest, IndividualEndpoint, NetworkAddress,
    RequestDestination,
};
use zb_core::{Destination, Endpoint, short_id};
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
const GROUP_BROADCAST_ADDRESS: u16 = short_id::Broadcast::AllDevices.as_u16();
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
    destination: Destination,
    source_endpoint: Endpoint,
    metadata: Metadata,
    asdu: Bytes,
) -> DataRequest<Bytes> {
    let destination = request_destination(destination);
    let source_endpoint = IndividualEndpoint::new(source_endpoint)
        .expect("coordinator transmissions use an individual source endpoint");

    DataRequest::new(
        destination,
        metadata.profile().as_u16(),
        metadata.cluster_id(),
        source_endpoint,
        asdu,
    )
    .with_tx_options(metadata.tx_options())
}

/// Convert a coordinator destination into APSDE request addressing.
#[must_use]
pub const fn request_destination(destination: Destination) -> RequestDestination {
    match destination {
        Destination::Device(device) => RequestDestination::Network {
            address: NetworkAddress::new(device.device().as_u16())
                .expect("device short addresses are valid APSDE network addresses"),
            endpoint: device.endpoint(),
        },
        Destination::Broadcast(broadcast) => RequestDestination::Broadcast {
            address: broadcast.address(),
            endpoint: broadcast.endpoint(),
        },
        Destination::Group(address) => RequestDestination::Group {
            address,
            broadcast_address: BroadcastAddress::new(GROUP_BROADCAST_ADDRESS)
                .expect("the all-devices address is a valid APSDE broadcast address"),
        },
    }
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
    responses: BTreeMap<u8, PendingConfirmation>,
    quarantined: BTreeSet<u8>,
}

#[derive(Debug)]
struct PendingConfirmation {
    generation: u64,
    response: PendingResponse,
}

impl TransmissionState {
    const fn new() -> Self {
        Self {
            next_counter: INITIAL_COUNTER,
            next_generation: INITIAL_GENERATION,
            responses: BTreeMap::new(),
            quarantined: BTreeSet::new(),
        }
    }

    /// Allocate a counter that cannot be confused with a pending or late confirmation.
    fn allocate(&mut self) -> Option<TransmissionToken> {
        for _ in 0..APS_COUNTER_COUNT {
            let counter = self.next_counter;
            self.next_counter = self.next_counter.wrapping_add(1);
            if !self.responses.contains_key(&counter) && !self.quarantined.contains(&counter) {
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
        let Some(pending) = self.responses.remove(&counter) else {
            if self.quarantined.remove(&counter) {
                log::debug!("Released quarantined APS counter after late confirmation: {counter}");
                return;
            }
            warn!("Received APS data confirmation for unknown counter: {counter}");
            return;
        };
        let result = if status.is_success() {
            Ok(())
        } else {
            Err(zb_hw::TransmissionError::Confirmation(status).into())
        };
        pending.response.send(result).unwrap_or_else(drop);
    }

    /// Store a response under its allocated Zigbee APS counter.
    fn store_pending_response(&mut self, token: TransmissionToken, response: PendingResponse) {
        let previous = self.responses.insert(
            token.counter,
            PendingConfirmation {
                generation: token.generation,
                response,
            },
        );
        debug_assert!(previous.is_none());
        debug_assert!(!self.quarantined.contains(&token.counter));
    }

    /// Complete an accepted transmission or retain it for its APS acknowledgement.
    fn handle_accepted_transmission(
        &mut self,
        token: TransmissionToken,
        acknowledged: bool,
        response: PendingResponse,
    ) {
        if acknowledged {
            self.store_pending_response(token, response);
        } else {
            response.send(Ok(())).unwrap_or_else(drop);
        }
    }

    /// Return a hardware rejection to the caller.
    fn handle_rejected_transmission(response: PendingResponse, error: zb_hw::Error) {
        response.send(Err(error)).unwrap_or_else(drop);
    }

    fn cancel(&mut self, token: TransmissionToken) {
        if self.pending_generation_matches(token) {
            self.responses.remove(&token.counter);
            self.quarantined.insert(token.counter);
        }
    }

    fn timeout(&mut self, token: TransmissionToken) {
        if !self.pending_generation_matches(token) {
            return;
        }
        let pending = self
            .responses
            .remove(&token.counter)
            .expect("matching pending generation remains present");
        self.quarantined.insert(token.counter);
        pending
            .response
            .send(Err(zb_hw::TransmissionError::Timeout.into()))
            .unwrap_or_else(drop);
    }

    fn pending_generation_matches(&self, token: TransmissionToken) -> bool {
        self.responses
            .get(&token.counter)
            .is_some_and(|pending| pending.generation == token.generation)
    }

    fn fail_all(&mut self, error: &zb_hw::Error) {
        let responses = std::mem::take(&mut self.responses);
        for (counter, pending) in responses {
            self.quarantined.insert(counter);
            pending
                .response
                .send(Err(error.clone()))
                .unwrap_or_else(drop);
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
            if !self.handle_actor_message(message).await {
                break;
            }
        }
    }

    async fn handle_actor_message(&mut self, message: Message) -> bool {
        match message {
            Message::Transmit { request, response } => {
                self.transmit(request, response).await;
            }
            Message::Confirm { counter, status } => {
                self.state.handle_confirm(counter, status);
            }
            Message::NetworkDown => {
                self.state
                    .fail_all(&zb_hw::TransmissionError::NoRoute.into());
            }
            Message::HardwareUnavailable => {
                self.state.fail_all(&zb_hw::Error::ActorUnavailable);
                return false;
            }
            Message::Cancel { token } => {
                self.state.cancel(token);
            }
            Message::ConfirmationTimeout { token } => {
                self.state.timeout(token);
            }
        }
        true
    }

    /// Assign an APS counter and submit a data-service request to the hardware actor.
    async fn transmit(
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

        match self.ncp.transmit(request, token.counter).await {
            Ok(()) => {
                self.state
                    .handle_accepted_transmission(token, acknowledged, completion);
                if acknowledged {
                    self.schedule_confirmation_timeout(token);
                }
            }
            Err(error) => TransmissionState::handle_rejected_transmission(completion, error),
        }
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
mod tests {
    use bytes::Bytes;
    use tokio::runtime::Runtime;
    use tokio::sync::mpsc::channel;
    use zb_aps::TxOptions;
    use zb_aps::apsde::{ConfirmStatus, Status as ApsStatus};
    use zb_core::destination::{Broadcast, Destination, Device};
    use zb_core::endpoint::Application;
    use zb_core::{Endpoint, GroupId, Profile, short_id};

    use super::{
        Aps, Message, PendingResponse, TransmissionState, TransmissionToken, acknowledged,
        data_request,
    };
    use crate::aps::{Metadata, TransmissionResponse};

    const CHANNEL_SIZE: usize = 1;
    const CLUSTER_ID: u16 = 0x1234;
    const DEVICE_ID: u16 = 0x1234;
    const FIRST_COUNTER: u8 = 1;
    const SECOND_GENERATION: u64 = 1;
    const GROUP_ID: u16 = 0x2345;
    const SECOND_COUNTER: u8 = 2;
    const PAYLOAD: &[u8] = &[0x12, 0x34];

    const fn application_endpoint() -> Endpoint {
        Endpoint::Application(Application::MIN)
    }

    fn unicast_destination() -> Destination {
        let device = short_id::Device::new(DEVICE_ID).expect("test device ID is valid");
        Device::new(device, application_endpoint()).into()
    }

    fn broadcast_destination() -> Destination {
        Broadcast::new(short_id::Broadcast::AllDevices, Endpoint::Broadcast).into()
    }

    fn group_destination() -> Destination {
        GroupId::new(GROUP_ID)
            .expect("test group ID is valid")
            .into()
    }

    const fn metadata() -> Metadata {
        Metadata::new(Profile::ZigbeeHomeAutomation, CLUSTER_ID)
    }

    fn request(
        destination: Destination,
        tx_options: TxOptions,
        payload: Bytes,
    ) -> zb_aps::apsde::DataRequest<Bytes> {
        data_request(destination, application_endpoint(), metadata(), payload)
            .with_tx_options(tx_options)
    }

    #[test]
    fn waits_for_backend_acceptance_for_unacknowledged_transmission() {
        Runtime::new()
            .expect("runtime must be available")
            .block_on(async {
                let (sender, mut receiver) = channel(CHANNEL_SIZE);
                let aps = Aps::new(sender);
                let metadata = metadata();

                let task = tokio::spawn(async move {
                    aps.transmit(
                        data_request(
                            unicast_destination(),
                            application_endpoint(),
                            metadata,
                            Bytes::from_static(PAYLOAD),
                        )
                        .with_tx_options(TxOptions::empty()),
                    )
                    .await
                });
                let Message::Transmit { request, response } =
                    receiver.recv().await.expect("message must be available")
                else {
                    panic!("expected APS transmit message");
                };
                assert_eq!(request.profile_id(), metadata.profile().as_u16());
                assert_eq!(request.cluster_id(), metadata.cluster_id());
                assert_eq!(request.asdu(), PAYLOAD);

                let (acceptance, transmission) = transmission_response(FIRST_COUNTER);
                response
                    .send(Ok(transmission))
                    .expect("APS response receiver must be available");
                let deferred = task
                    .await
                    .expect("task must complete")
                    .expect("APS actor channel must be available");
                let completion = tokio::spawn(deferred);
                assert!(!completion.is_finished());

                acceptance
                    .send(Ok(()))
                    .expect("completion remains available");
                assert!(
                    completion
                        .await
                        .expect("response task must complete")
                        .is_ok()
                );
            });
    }

    #[test]
    fn returns_deferred_hardware_response_for_acknowledged_transmission() {
        Runtime::new()
            .expect("runtime must be available")
            .block_on(async {
                let (sender, mut receiver) = channel(CHANNEL_SIZE);
                let aps = Aps::new(sender);
                let task = tokio::spawn(async move {
                    aps.transmit(request(
                        unicast_destination(),
                        TxOptions::ACKNOWLEDGED_TRANSMISSION,
                        Bytes::new(),
                    ))
                    .await
                });
                let Message::Transmit { response, .. } =
                    receiver.recv().await.expect("message must be available")
                else {
                    panic!("expected APS transmit message");
                };

                let (acceptance, transmission) = transmission_response(FIRST_COUNTER);
                response
                    .send(Ok(transmission))
                    .expect("APS response receiver must be available");
                let deferred = task
                    .await
                    .expect("task must complete")
                    .expect("APS actor channel must be available");
                let completion = tokio::spawn(deferred);
                assert!(!completion.is_finished());

                acceptance
                    .send(Ok(()))
                    .expect("completion remains available");

                assert!(
                    completion
                        .await
                        .expect("response task must complete")
                        .is_ok()
                );
            });
    }

    #[test]
    fn waits_for_backend_acceptance_for_acknowledged_non_unicast_transmissions() {
        Runtime::new()
            .expect("runtime must be available")
            .block_on(async {
                for destination in [group_destination(), broadcast_destination()] {
                    let (sender, mut receiver) = channel(CHANNEL_SIZE);
                    let aps = Aps::new(sender);

                    let task = tokio::spawn(async move {
                        aps.transmit(request(
                            destination,
                            TxOptions::ACKNOWLEDGED_TRANSMISSION,
                            Bytes::new(),
                        ))
                        .await
                    });
                    let Message::Transmit { response, .. } =
                        receiver.recv().await.expect("message must be available")
                    else {
                        panic!("expected APS transmit message");
                    };

                    let (acceptance, transmission) = transmission_response(FIRST_COUNTER);
                    response
                        .send(Ok(transmission))
                        .expect("APS response receiver must be available");
                    let deferred = task
                        .await
                        .expect("task must complete")
                        .expect("APS actor channel must be available");
                    let completion = tokio::spawn(deferred);
                    assert!(!completion.is_finished());

                    acceptance
                        .send(Ok(()))
                        .expect("completion remains available");
                    assert!(
                        completion
                            .await
                            .expect("response task must complete")
                            .is_ok()
                    );
                }
            });
    }

    #[test]
    fn counter_wrap_skips_pending_counters() {
        let mut state = TransmissionState::new();
        state.next_counter = u8::MAX;
        let (pending, _result) = tokio::sync::oneshot::channel();
        state.store_pending_response(transmission_token(u8::MIN), pending);

        assert_eq!(state.allocate().map(|token| token.counter), Some(u8::MAX));
        assert_eq!(
            state.allocate().map(|token| token.counter),
            Some(FIRST_COUNTER)
        );
    }

    #[test]
    fn data_request_preserves_transmission_fields() {
        let request = request(
            unicast_destination(),
            TxOptions::SECURITY_ENABLED,
            Bytes::from_static(PAYLOAD),
        );

        assert_eq!(request.source_endpoint().get(), application_endpoint());
        assert_eq!(request.profile_id(), Profile::ZigbeeHomeAutomation.as_u16());
        assert_eq!(request.cluster_id(), CLUSTER_ID);
        assert_eq!(request.tx_options(), TxOptions::SECURITY_ENABLED);
        assert_eq!(request.asdu(), PAYLOAD);
    }

    #[test]
    fn actor_awaits_acknowledgement_only_for_unicast_requests() {
        let unicast = request(
            unicast_destination(),
            TxOptions::ACKNOWLEDGED_TRANSMISSION,
            Bytes::new(),
        );
        let group = request(
            group_destination(),
            TxOptions::ACKNOWLEDGED_TRANSMISSION,
            Bytes::new(),
        );
        let broadcast = request(
            broadcast_destination(),
            TxOptions::ACKNOWLEDGED_TRANSMISSION,
            Bytes::new(),
        );

        assert!(acknowledged(&unicast));
        assert!(!acknowledged(&group));
        assert!(!acknowledged(&broadcast));
    }

    #[test]
    fn backend_acceptance_completes_unacknowledged_transmission() {
        Runtime::new()
            .expect("runtime must be available")
            .block_on(async {
                let mut state = TransmissionState::new();
                let (pending_response, _pending_result) = tokio::sync::oneshot::channel();
                let (response, result) = tokio::sync::oneshot::channel();
                state.store_pending_response(transmission_token(FIRST_COUNTER), pending_response);

                state.handle_accepted_transmission(
                    transmission_token(SECOND_COUNTER),
                    false,
                    response,
                );

                assert!(result.await.expect("response must be available").is_ok());
                assert!(state.responses.contains_key(&FIRST_COUNTER));
            });
    }

    #[test]
    fn backend_rejection_completes_transmission_with_error() {
        Runtime::new()
            .expect("runtime must be available")
            .block_on(async {
                let (response, result) = tokio::sync::oneshot::channel();

                TransmissionState::handle_rejected_transmission(
                    response,
                    zb_hw::TransmissionError::Rejected.into(),
                );

                assert!(matches!(
                    result.await.expect("response must be available"),
                    Err(zb_hw::Error::Transmission(
                        zb_hw::TransmissionError::Rejected
                    ))
                ));
            });
    }

    #[test]
    fn successful_confirmation_resolves_matching_transmission() {
        Runtime::new()
            .expect("runtime must be available")
            .block_on(async {
                let mut state = TransmissionState::new();
                let (response, result) = tokio::sync::oneshot::channel();
                state.store_pending_response(transmission_token(FIRST_COUNTER), response);

                state.handle_confirm(FIRST_COUNTER, ConfirmStatus::success());

                assert!(result.await.expect("response must be available").is_ok());
                assert!(state.responses.is_empty());
            });
    }

    #[test]
    fn unsuccessful_confirmation_resolves_matching_transmission() {
        Runtime::new()
            .expect("runtime must be available")
            .block_on(async {
                let mut state = TransmissionState::new();
                let (first_response, _first_result) = tokio::sync::oneshot::channel();
                let (second_response, second_result) = tokio::sync::oneshot::channel();
                state.store_pending_response(transmission_token(FIRST_COUNTER), first_response);
                state.store_pending_response(transmission_token(SECOND_COUNTER), second_response);

                let failure = ConfirmStatus::Aps(ApsStatus::NoAcknowledgement);
                state.handle_confirm(SECOND_COUNTER, failure);

                assert!(matches!(
                    second_result.await.expect("response must be available"),
                    Err(zb_hw::Error::Transmission(
                        zb_hw::TransmissionError::Confirmation(status)
                    )) if status == failure
                ));
                assert_eq!(state.responses.len(), CHANNEL_SIZE);
                assert!(state.responses.contains_key(&FIRST_COUNTER));
            });
    }

    #[test]
    fn allocator_does_not_replace_a_pending_transmission_after_wrap() {
        Runtime::new()
            .expect("runtime must be available")
            .block_on(async {
                let mut state = TransmissionState::new();
                let (previous_response, mut previous_result) = tokio::sync::oneshot::channel();
                state.store_pending_response(transmission_token(FIRST_COUNTER), previous_response);
                state.next_counter = FIRST_COUNTER;

                assert_eq!(
                    state.allocate().map(|token| token.counter),
                    Some(SECOND_COUNTER)
                );
                assert!(previous_result.try_recv().is_err());

                state.handle_confirm(FIRST_COUNTER, ConfirmStatus::success());
                assert!(
                    previous_result
                        .await
                        .expect("response is available")
                        .is_ok()
                );
            });
    }

    #[test]
    fn cancelled_counter_remains_quarantined_until_its_late_confirmation() {
        let mut state = TransmissionState::new();
        let token = transmission_token(FIRST_COUNTER);
        let (response, _result) = tokio::sync::oneshot::channel();
        state.store_pending_response(token, response);

        state.cancel(token);

        assert!(state.quarantined.contains(&FIRST_COUNTER));
        state.next_counter = FIRST_COUNTER;
        assert_eq!(
            state.allocate().map(|allocated| allocated.counter),
            Some(SECOND_COUNTER)
        );

        state.handle_confirm(FIRST_COUNTER, ConfirmStatus::success());

        assert!(!state.quarantined.contains(&FIRST_COUNTER));
        state.next_counter = FIRST_COUNTER;
        assert_eq!(
            state.allocate().map(|allocated| allocated.counter),
            Some(FIRST_COUNTER)
        );
    }

    #[test]
    fn stale_lifecycle_message_does_not_remove_reused_counter() {
        let mut state = TransmissionState::new();
        let stale_token = transmission_token(FIRST_COUNTER);
        let current = TransmissionToken {
            counter: FIRST_COUNTER,
            generation: SECOND_GENERATION,
        };
        let (response, mut result) = tokio::sync::oneshot::channel();
        state.store_pending_response(current, response);

        state.cancel(stale_token);
        state.timeout(stale_token);

        assert!(state.responses.contains_key(&FIRST_COUNTER));
        assert!(result.try_recv().is_err());
    }

    #[test]
    fn allocator_reports_exhaustion_when_every_counter_is_unavailable() {
        let mut state = TransmissionState::new();
        state.quarantined.extend(u8::MIN..=u8::MAX);

        assert_eq!(state.allocate(), None);
    }

    #[test]
    fn network_down_resolves_every_pending_transmission() {
        Runtime::new()
            .expect("runtime must be available")
            .block_on(async {
                let mut state = TransmissionState::new();
                let (first_response, first_result) = tokio::sync::oneshot::channel();
                let (second_response, second_result) = tokio::sync::oneshot::channel();
                state.store_pending_response(transmission_token(FIRST_COUNTER), first_response);
                state.store_pending_response(transmission_token(SECOND_COUNTER), second_response);

                state.fail_all(&zb_hw::TransmissionError::NoRoute.into());

                for result in [first_result, second_result] {
                    assert!(matches!(
                        result.await.expect("response is available"),
                        Err(zb_hw::Error::Transmission(
                            zb_hw::TransmissionError::NoRoute
                        ))
                    ));
                }
                assert!(state.responses.is_empty());
                assert!(state.quarantined.contains(&FIRST_COUNTER));
                assert!(state.quarantined.contains(&SECOND_COUNTER));
            });
    }

    #[test]
    fn hardware_unavailability_resolves_every_pending_transmission() {
        Runtime::new()
            .expect("runtime must be available")
            .block_on(async {
                let mut state = TransmissionState::new();
                let (first_response, first_result) = tokio::sync::oneshot::channel();
                let (second_response, second_result) = tokio::sync::oneshot::channel();
                state.store_pending_response(transmission_token(FIRST_COUNTER), first_response);
                state.store_pending_response(transmission_token(SECOND_COUNTER), second_response);

                state.fail_all(&zb_hw::Error::ActorUnavailable);

                for result in [first_result, second_result] {
                    assert!(matches!(
                        result.await.expect("response is available"),
                        Err(zb_hw::Error::ActorUnavailable)
                    ));
                }
                assert!(state.responses.is_empty());
                assert!(state.quarantined.contains(&FIRST_COUNTER));
                assert!(state.quarantined.contains(&SECOND_COUNTER));
            });
    }

    fn transmission_response(counter: u8) -> (PendingResponse, TransmissionResponse) {
        let (completion, result) = tokio::sync::oneshot::channel();
        let (inbox, _messages) = channel(CHANNEL_SIZE);
        (
            completion,
            TransmissionResponse::new(result, transmission_token(counter), inbox.downgrade()),
        )
    }

    const fn transmission_token(counter: u8) -> TransmissionToken {
        TransmissionToken {
            counter,
            generation: super::INITIAL_GENERATION,
        }
    }
}
