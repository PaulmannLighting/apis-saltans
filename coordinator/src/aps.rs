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
mod tests {
    use std::num::NonZeroUsize;
    use std::time::Duration;

    use bytes::Bytes;
    use tokio::runtime::Runtime;
    use tokio::sync::mpsc::channel;
    use zb_aps::TxOptions;
    use zb_aps::apsde::{
        BroadcastAddress, ConfirmStatus, IndividualEndpoint, NetworkAddress, NetworkDestination,
        RequestDestination, Status as ApsStatus,
    };
    use zb_core::endpoint::Application;
    use zb_core::short_id::Device as ShortDevice;
    use zb_core::{Endpoint, GroupId, IeeeAddress, Profile, short_id};
    use zb_hw::{
        ChannelMask, Driver, Error as HardwareError, FoundNetwork, Operation, ScanDuration,
        ScannedChannel,
    };
    use zb_zdp::SimpleDescriptor;

    use super::{
        Aps, INITIAL_COUNTER, INITIAL_GENERATION, Message, PendingResponse, Transceiver,
        TransmissionState, TransmissionToken, acknowledged, data_request,
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
    const NCP_CHANNEL_SIZE: NonZeroUsize = NonZeroUsize::MIN;
    const TEST_TIMEOUT: Duration = Duration::from_millis(100);

    #[derive(Debug)]
    struct DelayedDriver {
        acceptance: Option<tokio::sync::oneshot::Receiver<()>>,
    }

    impl Driver for DelayedDriver {
        async fn get_endpoints(&self) -> Result<Box<[SimpleDescriptor]>, HardwareError> {
            unsupported(Operation::GetEndpoints)
        }

        async fn get_pan_id(&mut self) -> Result<u16, HardwareError> {
            unsupported(Operation::GetPanId)
        }

        async fn get_ieee_address(&mut self) -> Result<IeeeAddress, HardwareError> {
            unsupported(Operation::GetIeeeAddress)
        }

        async fn scan_networks(
            &mut self,
            _channel_mask: ChannelMask,
            _duration: ScanDuration,
        ) -> Result<Vec<FoundNetwork>, HardwareError> {
            unsupported(Operation::ScanNetworks)
        }

        async fn scan_channels(
            &mut self,
            _channel_mask: ChannelMask,
            _duration: ScanDuration,
        ) -> Result<Vec<ScannedChannel>, HardwareError> {
            unsupported(Operation::ScanChannels)
        }

        async fn allow_joins(&mut self, _duration: Duration) -> Result<Duration, HardwareError> {
            unsupported(Operation::AllowJoins)
        }

        async fn route_request(&mut self, _radius: u8) -> Result<(), HardwareError> {
            unsupported(Operation::RouteRequest)
        }

        async fn short_id_to_ieee_address(
            &mut self,
            _short_id: ShortDevice,
        ) -> Result<IeeeAddress, HardwareError> {
            unsupported(Operation::ShortIdToIeeeAddress)
        }

        async fn ieee_address_to_short_id(
            &mut self,
            _ieee_address: IeeeAddress,
        ) -> Result<ShortDevice, HardwareError> {
            unsupported(Operation::IeeeAddressToShortId)
        }

        async fn transmit(
            &mut self,
            _request: zb_aps::apsde::DataRequest<Bytes>,
            _counter: u8,
        ) -> Result<(), HardwareError> {
            self.acceptance
                .take()
                .expect("test driver receives one transmission")
                .await
                .expect("test controls backend acceptance");
            Ok(())
        }
    }

    fn unsupported<T>(operation: Operation) -> Result<T, HardwareError> {
        Err(HardwareError::Unsupported(operation))
    }

    const fn application_endpoint() -> Endpoint {
        Endpoint::Application(Application::MIN)
    }

    const fn individual_endpoint() -> IndividualEndpoint {
        IndividualEndpoint::new(application_endpoint()).expect("application endpoint is individual")
    }

    fn unicast_destination() -> RequestDestination {
        NetworkDestination::new(
            NetworkAddress::new(DEVICE_ID).expect("test device ID is valid"),
            individual_endpoint(),
        )
        .into()
    }

    const fn broadcast_destination() -> RequestDestination {
        RequestDestination::Broadcast {
            address: short_id::Broadcast::AllDevices,
            endpoint: Endpoint::Broadcast,
        }
    }

    fn group_destination() -> RequestDestination {
        RequestDestination::Group {
            address: GroupId::new(GROUP_ID).expect("test group ID is valid"),
            broadcast_address: BroadcastAddress::new(short_id::Broadcast::AllDevices.as_u16())
                .expect("all-devices is a valid APSDE group broadcast selector"),
        }
    }

    const fn metadata() -> Metadata {
        Metadata::new(Profile::ZigbeeHomeAutomation, CLUSTER_ID)
    }

    fn request(
        destination: RequestDestination,
        tx_options: TxOptions,
        payload: Bytes,
    ) -> zb_aps::apsde::DataRequest<Bytes> {
        data_request(destination, individual_endpoint(), metadata(), payload)
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
                            individual_endpoint(),
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
    fn actor_routes_confirmation_while_backend_acceptance_is_pending() {
        Runtime::new()
            .expect("runtime must be available")
            .block_on(async {
                let (acceptance, accepted) = tokio::sync::oneshot::channel();
                let (ncp, driver) = DelayedDriver {
                    acceptance: Some(accepted),
                }
                .into_actor(NCP_CHANNEL_SIZE);
                let driver = tokio::spawn(driver);
                let aps = Transceiver::spawn(ncp);
                let transmission = aps
                    .transmit(request(
                        unicast_destination(),
                        TxOptions::ACKNOWLEDGED_TRANSMISSION,
                        Bytes::new(),
                    ))
                    .await
                    .expect("APS actor accepts the transmission");

                tokio::time::timeout(
                    TEST_TIMEOUT,
                    aps.confirm(INITIAL_COUNTER, ConfirmStatus::success()),
                )
                .await
                .expect("pending backend acceptance must not block confirmation routing")
                .expect("APS actor remains available");
                acceptance
                    .send(())
                    .expect("driver still waits for backend acceptance");
                tokio::time::timeout(TEST_TIMEOUT, transmission)
                    .await
                    .expect("buffered confirmation must resolve after backend acceptance")
                    .expect("transmission must succeed");

                drop(aps);
                tokio::time::timeout(TEST_TIMEOUT, driver)
                    .await
                    .expect("driver actor must stop after APS shutdown")
                    .expect("driver actor task must not panic");
            });
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
                let token = transmission_token(SECOND_COUNTER);
                state.store_submission(token, false, response);

                assert!(!state.finish_submission(token, Ok(())));

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
                let mut state = TransmissionState::new();
                let token = transmission_token(FIRST_COUNTER);
                state.store_submission(token, false, response);

                assert!(
                    !state
                        .finish_submission(token, Err(zb_hw::TransmissionError::Rejected.into()),)
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
    fn confirmation_can_arrive_before_backend_submission_completion() {
        Runtime::new()
            .expect("runtime must be available")
            .block_on(async {
                let mut state = TransmissionState::new();
                let token = transmission_token(FIRST_COUNTER);
                let (response, mut result) = tokio::sync::oneshot::channel();
                state.store_submission(token, true, response);

                state.handle_confirm(FIRST_COUNTER, ConfirmStatus::success());

                assert!(result.try_recv().is_err());
                assert!(!state.finish_submission(token, Ok(())));
                assert!(result.await.expect("response must be available").is_ok());
                assert!(state.responses.is_empty());
            });
    }

    #[test]
    fn cancellation_while_submitting_waits_for_acceptance_before_quarantine() {
        let mut state = TransmissionState::new();
        let token = transmission_token(FIRST_COUNTER);
        let (response, _result) = tokio::sync::oneshot::channel();
        state.store_submission(token, true, response);

        state.cancel(token);

        assert!(state.responses.contains_key(&FIRST_COUNTER));
        assert!(!state.quarantined.contains_key(&FIRST_COUNTER));
        assert!(state.finish_submission(token, Ok(())));
        assert!(!state.responses.contains_key(&FIRST_COUNTER));
        assert_eq!(
            state.quarantined.get(&FIRST_COUNTER),
            Some(&token.generation)
        );
    }

    #[test]
    fn cancelled_unacknowledged_submission_releases_counter_after_acceptance() {
        let mut state = TransmissionState::new();
        let token = transmission_token(FIRST_COUNTER);
        let (response, _result) = tokio::sync::oneshot::channel();
        state.store_submission(token, false, response);

        state.cancel(token);

        assert!(!state.finish_submission(token, Ok(())));
        assert!(!state.responses.contains_key(&FIRST_COUNTER));
        assert!(!state.quarantined.contains_key(&FIRST_COUNTER));
    }

    #[test]
    fn network_down_while_submitting_quarantines_after_acceptance() {
        Runtime::new()
            .expect("runtime must be available")
            .block_on(async {
                let mut state = TransmissionState::new();
                let token = transmission_token(FIRST_COUNTER);
                let (response, result) = tokio::sync::oneshot::channel();
                state.store_submission(token, true, response);

                state.network_down(&zb_hw::TransmissionError::NoRoute.into());

                assert!(matches!(
                    result.await.expect("response must be available"),
                    Err(zb_hw::Error::Transmission(
                        zb_hw::TransmissionError::NoRoute
                    ))
                ));
                assert!(state.responses.contains_key(&FIRST_COUNTER));
                assert!(state.finish_submission(token, Ok(())));
                assert_eq!(
                    state.quarantined.get(&FIRST_COUNTER),
                    Some(&token.generation)
                );
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

        assert!(state.quarantined.contains_key(&FIRST_COUNTER));
        state.next_counter = FIRST_COUNTER;
        assert_eq!(
            state.allocate().map(|allocated| allocated.counter),
            Some(SECOND_COUNTER)
        );

        state.handle_confirm(FIRST_COUNTER, ConfirmStatus::success());

        assert!(!state.quarantined.contains_key(&FIRST_COUNTER));
        assert!(!state.timeout(token));
        state.next_counter = FIRST_COUNTER;
        assert_eq!(
            state.allocate().map(|allocated| allocated.counter),
            Some(FIRST_COUNTER)
        );
    }

    #[test]
    fn missing_confirmation_at_deadline_is_terminal() {
        Runtime::new()
            .expect("runtime must be available")
            .block_on(async {
                let mut state = TransmissionState::new();
                let token = transmission_token(FIRST_COUNTER);
                let (response, result) = tokio::sync::oneshot::channel();
                state.store_pending_response(token, response);

                assert!(state.timeout(token));
                assert!(matches!(
                    result.await.expect("response is available"),
                    Err(zb_hw::Error::Transmission(
                        zb_hw::TransmissionError::Timeout
                    ))
                ));
            });
    }

    #[test]
    fn cancelled_transmission_with_missing_confirmation_is_terminal() {
        let mut state = TransmissionState::new();
        let token = transmission_token(FIRST_COUNTER);
        let (response, _result) = tokio::sync::oneshot::channel();
        state.store_pending_response(token, response);
        state.cancel(token);

        assert!(state.timeout(token));
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
        assert!(!state.timeout(stale_token));

        assert!(state.responses.contains_key(&FIRST_COUNTER));
        assert!(result.try_recv().is_err());
    }

    #[test]
    fn stale_timeout_does_not_match_newer_quarantine_generation() {
        let mut state = TransmissionState::new();
        let stale_token = transmission_token(FIRST_COUNTER);
        let current = TransmissionToken {
            counter: FIRST_COUNTER,
            generation: SECOND_GENERATION,
        };
        state
            .quarantined
            .insert(current.counter, current.generation);

        assert!(!state.timeout(stale_token));
        assert_eq!(
            state.quarantined.get(&current.counter),
            Some(&current.generation)
        );
    }

    #[test]
    fn allocator_reports_exhaustion_when_every_counter_is_unavailable() {
        let mut state = TransmissionState::new();
        state
            .quarantined
            .extend((u8::MIN..=u8::MAX).map(|counter| (counter, INITIAL_GENERATION)));

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

                state.network_down(&zb_hw::TransmissionError::NoRoute.into());

                for result in [first_result, second_result] {
                    assert!(matches!(
                        result.await.expect("response is available"),
                        Err(zb_hw::Error::Transmission(
                            zb_hw::TransmissionError::NoRoute
                        ))
                    ));
                }
                assert!(state.responses.is_empty());
                assert!(state.quarantined.contains_key(&FIRST_COUNTER));
                assert!(state.quarantined.contains_key(&SECOND_COUNTER));
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

                state.stop(&zb_hw::Error::ActorUnavailable);

                for result in [first_result, second_result] {
                    assert!(matches!(
                        result.await.expect("response is available"),
                        Err(zb_hw::Error::ActorUnavailable)
                    ));
                }
                assert!(state.responses.is_empty());
                assert!(state.quarantined.is_empty());
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
            generation: INITIAL_GENERATION,
        }
    }
}
