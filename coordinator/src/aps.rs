//! Actor for transmitting APS data frames.

use std::collections::BTreeMap;

use bytes::Bytes;
use log::warn;
use tokio::spawn;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::oneshot::channel;
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

const INITIAL_COUNTER: u8 = 0;
const GROUP_BROADCAST_ADDRESS: u16 = short_id::Broadcast::AllDevices.as_u16();

type PendingResponse = tokio::sync::oneshot::Sender<Result<(), zb_hw::Error>>;

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
    ) -> Result<TransmissionResponse, zb_hw::Error> {
        let (response, result) = channel();

        self.0
            .send(Message::Transmit { request, response })
            .await
            .map_err(|_| zb_hw::Error::ActorUnavailable)?;

        Ok(TransmissionResponse::new(result))
    }

    /// Forward a hardware APS data confirmation to the APS actor.
    pub async fn confirm(&self, counter: u8, status: ConfirmStatus) -> Result<(), zb_hw::Error> {
        self.0
            .send(Message::Confirm { counter, status })
            .await
            .map_err(|_| zb_hw::Error::ActorUnavailable)
    }
}

/// APS transmission actor.
#[derive(Debug)]
pub struct Transceiver {
    ncp: NcpHandle,
    state: TransmissionState,
}

#[derive(Debug)]
struct TransmissionState {
    counter: u8,
    responses: BTreeMap<u8, PendingResponse>,
}

impl TransmissionState {
    const fn new() -> Self {
        Self {
            counter: INITIAL_COUNTER,
            responses: BTreeMap::new(),
        }
    }

    /// Return and increment the APS frame counter.
    const fn next_counter(&mut self) -> u8 {
        let counter = self.counter;
        self.counter = self.counter.wrapping_add(1);
        counter
    }

    fn handle_confirm(&mut self, counter: u8, status: ConfirmStatus) {
        let Some(sender) = self.responses.remove(&counter) else {
            warn!("Received APS data confirmation for unknown counter: {counter}");
            return;
        };
        let result = if status.is_success() {
            Ok(())
        } else {
            Err(zb_hw::TransmissionError::Confirmation(status).into())
        };
        sender.send(result).unwrap_or_else(drop);
    }

    /// Store a response and time out the pending response it replaces, if any.
    fn store_pending_response(&mut self, counter: u8, response: PendingResponse) {
        let Some(pending_response) = self.responses.insert(counter, response) else {
            return;
        };
        pending_response
            .send(Err(zb_hw::TransmissionError::Timeout.into()))
            .unwrap_or_else(drop);
    }

    /// Complete an accepted transmission or retain it for its APS acknowledgement.
    fn handle_accepted_transmission(
        &mut self,
        counter: u8,
        acknowledged: bool,
        response: PendingResponse,
    ) {
        if acknowledged {
            self.store_pending_response(counter, response);
        } else {
            response.send(Ok(())).unwrap_or_else(drop);
        }
    }

    /// Return a hardware rejection to the caller.
    fn handle_rejected_transmission(response: PendingResponse, error: zb_hw::Error) {
        response.send(Err(error)).unwrap_or_else(drop);
    }
}

impl Transceiver {
    /// Create an APS actor with its frame counter initialized to zero.
    #[must_use]
    pub const fn new(ncp: NcpHandle) -> Self {
        Self {
            ncp,
            state: TransmissionState::new(),
        }
    }

    /// Run the APS actor.
    pub async fn run(mut self, mut messages: Receiver<Message>) {
        while let Some(message) = messages.recv().await {
            match message {
                Message::Transmit { request, response } => {
                    self.transmit(request, response).await;
                }
                Message::Confirm { counter, status } => {
                    self.state.handle_confirm(counter, status);
                }
            }
        }
    }

    /// Assign an APS counter and submit a data-service request to the hardware actor.
    async fn transmit(&mut self, request: DataRequest<Bytes>, response: PendingResponse) {
        let acknowledged = acknowledged(&request);
        let counter = self.state.next_counter();

        match self.ncp.transmit(request, counter).await {
            Ok(()) => {
                self.state
                    .handle_accepted_transmission(counter, acknowledged, response);
            }
            Err(error) => TransmissionState::handle_rejected_transmission(response, error),
        }
    }

    /// Spawn the APS actor.
    pub fn spawn(ncp: NcpHandle) -> Aps {
        let (aps_tx, aps_rx) = tokio::sync::mpsc::channel(MPSC_CHANNEL_SIZE);
        spawn(Self::new(ncp).run(aps_rx));
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

    use super::{Aps, Message, TransmissionState, acknowledged, data_request};
    use crate::aps::Metadata;

    const CHANNEL_SIZE: usize = 1;
    const CLUSTER_ID: u16 = 0x1234;
    const DEVICE_ID: u16 = 0x1234;
    const FIRST_COUNTER: u8 = 1;
    const GROUP_ID: u16 = 0x2345;
    const SECOND_COUNTER: u8 = 2;
    const LAST_COUNTER: u8 = u8::MAX;
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

                let deferred = task
                    .await
                    .expect("task must complete")
                    .expect("APS actor channel must be available");
                let completion = tokio::spawn(deferred);
                assert!(!completion.is_finished());

                response
                    .send(Ok(()))
                    .expect("APS response receiver must be available");
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

                let deferred = task
                    .await
                    .expect("task must complete")
                    .expect("APS actor channel must be available");
                let completion = tokio::spawn(deferred);
                assert!(!completion.is_finished());

                response
                    .send(Ok(()))
                    .expect("APS response receiver must be available");

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

                    let deferred = task
                        .await
                        .expect("task must complete")
                        .expect("APS actor channel must be available");
                    let completion = tokio::spawn(deferred);
                    assert!(!completion.is_finished());

                    response
                        .send(Ok(()))
                        .expect("APS response receiver must be available");
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
    fn counter_wraps_after_its_maximum_value() {
        let mut state = TransmissionState::new();
        state.counter = LAST_COUNTER;

        assert_eq!(state.next_counter(), LAST_COUNTER);
        assert_eq!(state.next_counter(), 0);
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
                state.responses.insert(FIRST_COUNTER, pending_response);

                state.handle_accepted_transmission(FIRST_COUNTER, false, response);

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
                state.responses.insert(FIRST_COUNTER, response);

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
                state.responses.insert(FIRST_COUNTER, first_response);
                state.responses.insert(SECOND_COUNTER, second_response);

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
    fn replacing_pending_transmission_times_out_previous_transmission() {
        Runtime::new()
            .expect("runtime must be available")
            .block_on(async {
                let mut state = TransmissionState::new();
                let (previous_response, previous_result) = tokio::sync::oneshot::channel();
                let (replacement_response, replacement_result) = tokio::sync::oneshot::channel();
                state.responses.insert(LAST_COUNTER, previous_response);

                state.store_pending_response(LAST_COUNTER, replacement_response);
                assert!(matches!(
                    previous_result.await.expect("response must be available"),
                    Err(zb_hw::Error::Transmission(
                        zb_hw::TransmissionError::Timeout
                    ))
                ));

                state.handle_confirm(LAST_COUNTER, ConfirmStatus::success());
                assert!(
                    replacement_result
                        .await
                        .expect("replacement response must be available")
                        .is_ok()
                );
            });
    }
}
