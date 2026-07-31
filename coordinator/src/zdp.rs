//! Transceiver to send and receive ZDP messages.

use std::collections::BTreeMap;

use bytes::Bytes;
use le_stream::ToLeStream;
use log::{debug, error, trace, warn};
use tokio::runtime::Handle;
use tokio::spawn;
use tokio::sync::mpsc::error::TrySendError;
use tokio::sync::mpsc::{Receiver, Sender, WeakSender};
use tokio::task::AbortHandle;
use tokio::time::sleep;
use zb_aps::apsde::{DataIndication, DataRequest, NetworkAddress, ReceivedDestination};
use zb_core::node::Descriptor;
use zb_core::short_id::Device;
use zb_core::{
    ClusterSpecific, Destination, Endpoint, FullAddress, IeeeAddress, Profile, destination,
};
use zb_hw::NcpHandle;
use zb_zdp::{
    ActiveEpReq, ActiveEpRsp, Command, DeviceAndServiceDiscovery, DeviceAnnce, Frame, IeeeAddrReq,
    IeeeAddrRsp, IeeeAddrRspResponse, MatchDescReq, MatchDescRsp, MgmtPermitJoiningRsp,
    NetworkManagement, NodeDescReq, NodeDescRsp, NwkAddrReq, NwkAddrRsp, NwkAddrRspResponse,
    PowerDescReq, PowerDescRsp, RequestType, SimpleDescReq, SimpleDescRsp, Status,
    SystemServerDiscoveryReq, SystemServerDiscoveryRsp,
};

use self::discovery::{
    DescriptorTarget, LOCAL_NWK_ADDRESS, active_endpoints, descriptor_target, matching_server_mask,
    simple_descriptor,
};
use self::match_desc::{
    Action as MatchDescAction, action as match_desc_action, local_response as local_match_response,
    matching_endpoints,
};
pub use self::message::Message;
use self::node_desc::{
    Action as NodeDescAction, action as node_desc_action, unavailable_child_status,
};
use crate::aps::{Aps, Metadata, TransmissionResponse};
use crate::correlation::{
    Cancellation, Key, PROTOCOL_QUARANTINE_TIMEOUT, PROTOCOL_RESPONSE_TIMEOUT, Registry, Token,
};
use crate::event::EventSink;
use crate::response::ApsProtocolResponse;
use crate::{Device as DeviceEvent, Event, MPSC_CHANNEL_SIZE};

mod discovery;
mod match_desc;
mod message;
mod node_desc;

const INITIAL_SERVER_OPERATION_ID: u64 = 0;
const INITIAL_COMMUNICATION_SUBMISSION_ID: u64 = 0;
const COMMUNICATION_SUBMISSION_LIMIT: usize = MPSC_CHANNEL_SIZE;
const SERVER_OPERATION_LIMIT: usize = MPSC_CHANNEL_SIZE;

/// Zigbee transceiver actor.
#[derive(Debug)]
pub struct Transceiver {
    server: Server,
    events: EventSink,
    responses: Registry<Command>,
    inbox: WeakSender<Message>,
    communication_submissions: BTreeMap<u64, CommunicationSubmission>,
    next_communication_submission_id: u64,
    server_operations: BTreeMap<u64, AbortHandle>,
    next_server_operation_id: u64,
}

/// Actor-owned state retained while a ZDP request is being handed to APS.
#[derive(Debug)]
struct CommunicationSubmission {
    token: Token,
    protocol_response: tokio::sync::oneshot::Receiver<Result<Command, crate::Error>>,
    response: tokio::sync::oneshot::Sender<Result<ApsProtocolResponse<Command>, crate::Error>>,
    task: AbortHandle,
}

/// Cloneable context used by bounded background ZDP request-serving operations.
#[derive(Clone, Debug)]
struct Server {
    ncp: NcpHandle,
    aps: Aps,
    descriptor: Descriptor,
    inbox: WeakSender<Message>,
}

/// A received ZDP request that may require asynchronous NCP or APS work.
#[derive(Debug)]
struct ServerRequest {
    source: NetworkAddress,
    request_was_broadcast: bool,
    sequence: u8,
    command: Command,
}

impl Transceiver {
    /// Create a new transceiver.
    #[must_use]
    pub fn new(
        ncp: NcpHandle,
        aps: Aps,
        events: EventSink,
        descriptor: Descriptor,
        inbox: WeakSender<Message>,
    ) -> Self {
        Self {
            server: Server {
                ncp,
                aps,
                descriptor,
                inbox: inbox.clone(),
            },
            events,
            responses: Registry::new(),
            inbox,
            communication_submissions: BTreeMap::new(),
            next_communication_submission_id: INITIAL_COMMUNICATION_SUBMISSION_ID,
            server_operations: BTreeMap::new(),
            next_server_operation_id: INITIAL_SERVER_OPERATION_ID,
        }
    }

    /// Run the transceiver.
    pub async fn run(mut self, mut messages: Receiver<Message>) {
        while let Some(message) = messages.recv().await {
            if !self.handle_actor_message(message) {
                break;
            }
        }
        self.abort_server_operations();
        self.abort_communication_submissions();
    }

    fn handle_actor_message(&mut self, message: Message) -> bool {
        match message {
            Message::Received { indication } => {
                self.handle_message_received(indication);
            }
            Message::NetworkDown => {
                self.abort_server_operations();
                self.handle_network_down();
            }
            Message::HardwareUnavailable => {
                self.abort_server_operations();
                self.fail_communication_submissions_for_hardware_unavailability();
                self.responses.hardware_unavailable();
                return false;
            }
            Message::Cancel { token } => {
                if self.responses.cancel(token) {
                    self.schedule_quarantine_timeout(token);
                }
            }
            Message::ResponseTimeout { token } => {
                if self.responses.timeout(token) {
                    self.schedule_quarantine_timeout(token);
                }
            }
            Message::QuarantineTimeout { token } => {
                self.responses.expire_quarantine(token);
            }
            Message::ReplyTransmissionFailed { error } => {
                error!("ZDP server response transmission failed: {error}");
            }
            Message::ServerOperationFinished { id } => {
                if self.server_operations.remove(&id).is_none() {
                    debug!("Ignoring completion for unknown ZDP server operation {id}");
                }
            }
            Message::CommunicationSubmissionFinished { id, result } => {
                let Some(submission) = self.communication_submissions.remove(&id) else {
                    debug!("Ignoring completion for unknown ZDP communication submission {id}");
                    return true;
                };
                let CommunicationSubmission {
                    token,
                    protocol_response,
                    response,
                    task: _,
                } = submission;
                let result = match result {
                    Ok(transmission) => Ok(ApsProtocolResponse::new(
                        transmission,
                        protocol_response,
                        self.cancellation(token),
                    )),
                    Err(error) => {
                        self.responses.discard(token);
                        Err(error)
                    }
                };
                response.send(result).unwrap_or_else(drop);
            }
            Message::Communicate {
                device,
                request,
                response,
            } => {
                self.communicate(device, request, response);
            }
        }
        true
    }

    fn handle_message_received(&mut self, indication: DataIndication<Frame<Command>, (), ()>) {
        let Some((source_address, key)) = received_key(&indication) else {
            return;
        };
        let request_was_broadcast = matches!(
            indication.metadata().destination(),
            ReceivedDestination::Broadcast { .. }
        );
        trace!("Received ZDP message: {indication:?}");
        let (_, zdp_frame) = indication.into_parts();
        let (seq, command) = zdp_frame.into_parts();

        if let Command::DeviceAndServiceDiscovery(DeviceAndServiceDiscovery::DeviceAnnce(
            device_annce,
        )) = &command
        {
            handle_device_annce(&self.events, device_annce.as_ref());
            return;
        }
        if is_server_request(&command) {
            self.spawn_server_operation(ServerRequest {
                source: source_address,
                request_was_broadcast,
                sequence: seq,
                command,
            });
            return;
        }

        if self.responses.complete(key, command.clone()) {
            debug!(
                "Answering ZDP request: seq={seq} cluster_id={:#06X}",
                command.cluster_id()
            );
        } else if self.responses.release_quarantine(key) {
            debug!("Discarding late ZDP response with quarantined sequence {seq}");
        } else {
            warn!("Unexpected ZDP response: {command:?}");
        }
    }

    fn spawn_server_operation(&mut self, request: ServerRequest) {
        if self.server_operations.len() >= SERVER_OPERATION_LIMIT {
            warn!(
                "Discarding ZDP server request because the operation limit of \
                 {SERVER_OPERATION_LIMIT} has been reached"
            );
            return;
        }

        let id = self.allocate_server_operation_id();
        let server = self.server.clone();
        let inbox = self.inbox.clone();
        let task = spawn(async move {
            server.handle(request).await;
            let Some(inbox) = inbox.upgrade() else {
                return;
            };
            inbox
                .send(Message::ServerOperationFinished { id })
                .await
                .unwrap_or_else(|error| {
                    debug!("Failed to retire ZDP server operation: {error}");
                });
        });
        let previous = self.server_operations.insert(id, task.abort_handle());
        debug_assert!(previous.is_none());
    }

    fn abort_server_operations(&mut self) {
        for operation in std::mem::take(&mut self.server_operations).into_values() {
            operation.abort();
        }
    }

    fn allocate_server_operation_id(&mut self) -> u64 {
        loop {
            let id = self.next_server_operation_id;
            self.next_server_operation_id = self.next_server_operation_id.wrapping_add(1);
            if !self.server_operations.contains_key(&id) {
                return id;
            }
        }
    }

    /// Send a ZDP unicast message with back-channel communication.
    ///
    /// # Returns
    ///
    /// The result is returned through `response` after the request has been handed to the APS
    /// actor.
    fn communicate(
        &mut self,
        device: Device,
        request: DataRequest<Bytes>,
        response: tokio::sync::oneshot::Sender<Result<ApsProtocolResponse<Command>, crate::Error>>,
    ) {
        if self.communication_submissions.len() >= COMMUNICATION_SUBMISSION_LIMIT {
            warn!(
                "Rejecting ZDP communication because the submission limit of \
                 {COMMUNICATION_SUBMISSION_LIMIT} has been reached"
            );
            response
                .send(Err(crate::Error::SendError))
                .unwrap_or_else(drop);
            return;
        }
        let (seq, token, protocol_response) = match self
            .responses
            .register(|sequence| Key::from_zdp_command(device, sequence, &request))
        {
            Ok(registration) => registration,
            Err(error) => {
                response.send(Err(error)).unwrap_or_else(drop);
                return;
            }
        };
        self.schedule_response_timeout(token);
        let request = request.map_asdu(|payload| Frame::new(seq, payload).to_le_stream().collect());
        let aps = self.server.aps.clone();
        let inbox = self.inbox.clone();
        let id = self.allocate_communication_submission_id();
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
        let previous = self.communication_submissions.insert(
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

    fn handle_network_down(&mut self) {
        let submissions = std::mem::take(&mut self.communication_submissions);
        let protected = submissions
            .values()
            .map(|submission| submission.token)
            .collect::<Vec<_>>();
        let quarantined = self
            .responses
            .network_down_preserving(&zb_hw::TransmissionError::NoRoute, protected);
        for token in quarantined {
            self.schedule_quarantine_timeout(token);
        }

        for submission in submissions.into_values() {
            submission.task.abort();
            submission
                .response
                .send(Err(
                    zb_hw::Error::from(zb_hw::TransmissionError::NoRoute).into()
                ))
                .unwrap_or_else(drop);
        }
    }

    fn fail_communication_submissions_for_hardware_unavailability(&mut self) {
        for submission in std::mem::take(&mut self.communication_submissions).into_values() {
            submission.task.abort();
            submission
                .response
                .send(Err(zb_hw::Error::ActorUnavailable.into()))
                .unwrap_or_else(drop);
        }
    }

    fn abort_communication_submissions(&mut self) {
        for submission in std::mem::take(&mut self.communication_submissions).into_values() {
            submission.task.abort();
        }
    }

    fn allocate_communication_submission_id(&mut self) -> u64 {
        loop {
            let id = self.next_communication_submission_id;
            self.next_communication_submission_id =
                self.next_communication_submission_id.wrapping_add(1);
            if !self.communication_submissions.contains_key(&id) {
                return id;
            }
        }
    }

    fn cancellation(&self, token: Token) -> Cancellation {
        let inbox = self.inbox.clone();
        let runtime = Handle::current();
        Cancellation::new(token, move |token| {
            let Some(inbox) = inbox.upgrade() else {
                return;
            };
            match inbox.try_send(Message::Cancel { token }) {
                Ok(()) => {}
                Err(TrySendError::Full(message)) => {
                    runtime.spawn(async move {
                        inbox.send(message).await.unwrap_or_else(|error| {
                            debug!("Failed to enqueue ZDP response cancellation: {error}");
                        });
                    });
                }
                Err(TrySendError::Closed(_)) => {
                    debug!("Failed to enqueue ZDP response cancellation: actor unavailable");
                }
            }
        })
    }

    fn schedule_response_timeout(&self, token: Token) {
        let inbox = self.inbox.clone();
        spawn(async move {
            sleep(PROTOCOL_RESPONSE_TIMEOUT).await;
            let Some(inbox) = inbox.upgrade() else {
                return;
            };
            inbox
                .send(Message::ResponseTimeout { token })
                .await
                .unwrap_or_else(|error| {
                    debug!("Failed to enqueue ZDP response timeout: {error}");
                });
        });
    }

    fn schedule_quarantine_timeout(&self, token: Token) {
        let inbox = self.inbox.clone();
        spawn(async move {
            sleep(PROTOCOL_QUARANTINE_TIMEOUT).await;
            let Some(inbox) = inbox.upgrade() else {
                return;
            };
            inbox
                .send(Message::QuarantineTimeout { token })
                .await
                .unwrap_or_else(|error| {
                    debug!("Failed to enqueue ZDP quarantine timeout: {error}");
                });
        });
    }

    /// Start the ZDP transceiver.
    pub fn spawn(
        ncp: NcpHandle,
        aps: Aps,
        events: EventSink,
        descriptor: Descriptor,
    ) -> Sender<Message> {
        let (zdp_tx, zdp_rx) = tokio::sync::mpsc::channel(MPSC_CHANNEL_SIZE);
        spawn(Self::new(ncp, aps, events, descriptor, zdp_tx.downgrade()).run(zdp_rx));
        zdp_tx
    }
}

impl Server {
    async fn handle(&self, request: ServerRequest) {
        let ServerRequest {
            source,
            request_was_broadcast,
            sequence,
            command,
        } = request;

        match command {
            Command::DeviceAndServiceDiscovery(DeviceAndServiceDiscovery::NwkAddrReq(request)) => {
                self.handle_nwk_addr_req(source, sequence, *request).await;
            }
            Command::DeviceAndServiceDiscovery(DeviceAndServiceDiscovery::IeeeAddrReq(request)) => {
                self.handle_ieee_addr_req(source, sequence, *request).await;
            }
            Command::DeviceAndServiceDiscovery(DeviceAndServiceDiscovery::PowerDescReq(
                request,
            )) => {
                self.handle_power_desc_req(source, sequence, *request).await;
            }
            Command::DeviceAndServiceDiscovery(DeviceAndServiceDiscovery::SimpleDescReq(
                request,
            )) => {
                self.handle_simple_desc_req(source, sequence, *request)
                    .await;
            }
            Command::DeviceAndServiceDiscovery(DeviceAndServiceDiscovery::ActiveEpReq(request)) => {
                self.handle_active_ep_req(source, sequence, *request).await;
            }
            Command::DeviceAndServiceDiscovery(DeviceAndServiceDiscovery::MatchDescReq(
                request,
            )) => {
                self.handle_match_desc_req(source, request_was_broadcast, sequence, *request)
                    .await;
            }
            Command::DeviceAndServiceDiscovery(DeviceAndServiceDiscovery::NodeDescReq(request)) => {
                self.handle_node_desc_req(source, sequence, *request).await;
            }
            Command::DeviceAndServiceDiscovery(
                DeviceAndServiceDiscovery::SystemServerDiscoveryReq(request),
            ) => {
                self.handle_system_server_discovery_req(source, sequence, *request)
                    .await;
            }
            Command::NetworkManagement(NetworkManagement::MgmtPermitJoiningReq(_)) => {
                self.handle_mgmt_permit_joining_req(source, request_was_broadcast, sequence)
                    .await;
            }
            _ => unreachable!("only recognized ZDP server requests are spawned"),
        }
    }

    async fn handle_nwk_addr_req(&self, source: NetworkAddress, seq: u8, request: NwkAddrReq) {
        let response = match request.request_type() {
            Ok(RequestType::SingleDeviceResponse) => self
                .resolve_nwk_address(request.ieee_addr())
                .await
                .map(|nwk_addr_remote_dev| NwkAddrRspResponse::Single {
                    ieee_addr_remote_dev: request.ieee_addr(),
                    nwk_addr_remote_dev,
                }),
            Ok(RequestType::ExtendedResponse) => Err(Status::NotSupported),
            Err(_) => Err(Status::InvalidRequestType),
        };

        self.respond_to_source(source, seq, NwkAddrRsp::new(response), "NWK_addr_rsp")
            .await;
    }

    async fn handle_ieee_addr_req(&self, source: NetworkAddress, seq: u8, request: IeeeAddrReq) {
        let response = match RequestType::try_from(request.request_type()) {
            Ok(RequestType::SingleDeviceResponse) => self
                .resolve_ieee_address(request.nwk_addr_of_interest())
                .await
                .map(|ieee_addr_remote_dev| IeeeAddrRspResponse::Single {
                    ieee_addr_remote_dev,
                    nwk_addr_remote_dev: request.nwk_addr_of_interest(),
                }),
            Ok(RequestType::ExtendedResponse) => Err(Status::NotSupported),
            Err(_) => Err(Status::InvalidRequestType),
        };

        self.respond_to_source(source, seq, IeeeAddrRsp::new(response), "IEEE_addr_rsp")
            .await;
    }

    async fn handle_power_desc_req(&self, source: NetworkAddress, seq: u8, request: PowerDescReq) {
        let nwk_addr_of_interest = request.nwk_addr_of_interest();
        let power_descriptor = match descriptor_target(nwk_addr_of_interest) {
            DescriptorTarget::Local => Err(Status::NoDescriptor),
            DescriptorTarget::Remote(device) => Err(self.remote_descriptor_status(device).await),
            DescriptorTarget::Invalid => Err(Status::InvalidRequestType),
        };
        let response = PowerDescRsp::new(nwk_addr_of_interest, power_descriptor);

        self.respond_to_source(source, seq, response, "Power_Desc_rsp")
            .await;
    }

    async fn handle_simple_desc_req(
        &self,
        source: NetworkAddress,
        seq: u8,
        request: SimpleDescReq,
    ) {
        let nwk_addr_of_interest = request.nwk_address_of_interest();
        let descriptor = match descriptor_target(nwk_addr_of_interest) {
            DescriptorTarget::Local => match self.ncp.get_endpoints().await {
                Ok(descriptors) => simple_descriptor(request.endpoint(), &descriptors),
                Err(error) => {
                    error!("Failed to read local endpoints for Simple_Desc_req: {error}");
                    Err(Status::NoDescriptor)
                }
            },
            DescriptorTarget::Remote(device) => Err(self.remote_descriptor_status(device).await),
            DescriptorTarget::Invalid => Err(Status::InvalidRequestType),
        };
        let response = SimpleDescRsp::new(nwk_addr_of_interest, descriptor);

        self.respond_to_source(source, seq, response, "Simple_Desc_rsp")
            .await;
    }

    async fn handle_active_ep_req(&self, source: NetworkAddress, seq: u8, request: ActiveEpReq) {
        let nwk_addr_of_interest = request.nwk_addr_of_interest();
        let endpoints = match descriptor_target(nwk_addr_of_interest) {
            DescriptorTarget::Local => match self.ncp.get_endpoints().await {
                Ok(descriptors) => active_endpoints(&descriptors),
                Err(error) => {
                    error!("Failed to read local endpoints for Active_EP_req: {error}");
                    Err(Status::NoDescriptor)
                }
            },
            DescriptorTarget::Remote(device) => Err(self.remote_descriptor_status(device).await),
            DescriptorTarget::Invalid => Err(Status::InvalidRequestType),
        };
        let response = ActiveEpRsp::new(nwk_addr_of_interest, endpoints);

        self.respond_to_source(source, seq, response, "Active_EP_rsp")
            .await;
    }

    async fn handle_system_server_discovery_req(
        &self,
        source: NetworkAddress,
        seq: u8,
        request: SystemServerDiscoveryReq,
    ) {
        let Some(server_mask) = matching_server_mask(request.server_mask(), &self.descriptor)
        else {
            return;
        };
        let response = SystemServerDiscoveryRsp::new(Status::Success.into(), server_mask.bits());

        self.respond_to_source(source, seq, response, "System_Server_Discovery_rsp")
            .await;
    }

    async fn resolve_nwk_address(&self, ieee_address: IeeeAddress) -> Result<u16, Status> {
        match self.ncp.get_ieee_address().await {
            Ok(local_address) if local_address == ieee_address => Ok(LOCAL_NWK_ADDRESS),
            Ok(_) => self.resolve_remote_nwk_address(ieee_address).await,
            Err(error) => {
                error!("Failed to read the coordinator IEEE address: {error}");
                self.resolve_remote_nwk_address(ieee_address).await
            }
        }
    }

    async fn resolve_remote_nwk_address(&self, ieee_address: IeeeAddress) -> Result<u16, Status> {
        self.ncp
            .ieee_address_to_short_id(ieee_address)
            .await
            .map(Device::as_u16)
            .map_err(|_| Status::DeviceNotFound)
    }

    async fn resolve_ieee_address(&self, nwk_address: u16) -> Result<IeeeAddress, Status> {
        match descriptor_target(nwk_address) {
            DescriptorTarget::Local => self.ncp.get_ieee_address().await,
            DescriptorTarget::Remote(device) => self.ncp.short_id_to_ieee_address(device).await,
            DescriptorTarget::Invalid => return Err(Status::InvalidRequestType),
        }
        .map_err(|_| Status::DeviceNotFound)
    }

    async fn remote_descriptor_status(&self, device: Device) -> Status {
        let device_is_known = self.ncp.short_id_to_ieee_address(device).await.is_ok();
        unavailable_child_status(device_is_known)
    }

    async fn respond_to_source<T>(
        &self,
        source: NetworkAddress,
        seq: u8,
        response: T,
        response_name: &str,
    ) where
        T: ClusterSpecific + ToLeStream,
    {
        let Ok(node_id) = source.as_u16().try_into().inspect_err(|error| {
            warn!("Invalid node ID: {error:?}");
        }) else {
            return;
        };

        if let Err(error) = self.respond(seq, node_id, response).await {
            error!("Failed to send {response_name}: {error:?}");
        }
    }

    async fn respond<T>(&self, seq: u8, device: Device, payload: T) -> Result<(), crate::Error>
    where
        T: ClusterSpecific + ToLeStream,
    {
        let destination = Destination::Device(destination::Device::new(device, Endpoint::Data));
        let request = crate::aps::data_request(
            destination,
            Endpoint::Data,
            Metadata::new(Profile::Network, T::ID),
            Frame::new(seq, payload).to_le_stream().collect(),
        );
        let transmission = self.aps.transmit(request).await?;
        track_reply_completion(transmission, self.inbox.clone());
        Ok(())
    }

    /// Process a Match Descriptor request and unicast any required response to its originator.
    async fn handle_match_desc_req(
        &self,
        source: NetworkAddress,
        request_was_broadcast: bool,
        seq: u8,
        match_desc_req: MatchDescReq,
    ) {
        let Ok(logical_type) = self
            .descriptor
            .flags()
            .logical_type()
            .inspect_err(|value| warn!("Invalid logical device type: {value:#04X}"))
        else {
            return;
        };

        let nwk_addr_of_interest = match_desc_req.nwk_addr_of_interest();

        let response =
            match match_desc_action(logical_type, nwk_addr_of_interest, request_was_broadcast) {
                MatchDescAction::MatchLocalDescriptors => {
                    let Ok(endpoints) = self.ncp.get_endpoints().await else {
                        return;
                    };
                    let Some(matches) = matching_endpoints(&match_desc_req, &endpoints) else {
                        error!("Too many endpoints matched Match_Desc_req");
                        return;
                    };

                    let Some(response) =
                        local_match_response(nwk_addr_of_interest, matches, request_was_broadcast)
                    else {
                        return;
                    };
                    response
                }
                MatchDescAction::MatchRemoteDevice(nwk_address) => {
                    if self
                        .ncp
                        .short_id_to_ieee_address(nwk_address)
                        .await
                        .is_err()
                    {
                        MatchDescRsp::new(nwk_addr_of_interest, Err(Status::DeviceNotFound))
                    } else if request_was_broadcast {
                        return;
                    } else {
                        MatchDescRsp::new(nwk_addr_of_interest, Err(Status::NoDescriptor))
                    }
                }
                MatchDescAction::RespondWithError(status) => {
                    MatchDescRsp::new(nwk_addr_of_interest, Err(status))
                }
                MatchDescAction::Ignore => return,
            };

        let Ok(node_id) = source.as_u16().try_into().inspect_err(|error| {
            warn!("Invalid node ID: {error:?}");
        }) else {
            return;
        };

        if let Err(error) = self.respond(seq, node_id, response).await {
            error!("Failed to send Match_Desc_rsp: {error:?}");
        }
    }

    /// Respond to a Node Descriptor request with the descriptor or an appropriate status.
    async fn handle_node_desc_req(
        &self,
        source: NetworkAddress,
        seq: u8,
        node_desc_req: NodeDescReq,
    ) {
        let Ok(node_id) = source.as_u16().try_into().inspect_err(|error| {
            warn!("Invalid node ID: {error:?}");
        }) else {
            return;
        };

        let Ok(logical_type) = self
            .descriptor
            .flags()
            .logical_type()
            .inspect_err(|value| warn!("Invalid logical device type: {value:#04X}"))
        else {
            return;
        };

        let nwk_addr_of_interest = node_desc_req.nwk_addr();
        let node_descriptor = match node_desc_action(logical_type, nwk_addr_of_interest) {
            NodeDescAction::RespondWithLocalDescriptor => Ok(self.descriptor.clone()),
            NodeDescAction::ResolveChild(nwk_address) => {
                let child_is_known = self.ncp.short_id_to_ieee_address(nwk_address).await.is_ok();

                Err(unavailable_child_status(child_is_known))
            }
            NodeDescAction::RespondWithError(status) => Err(status),
        };
        let payload = NodeDescRsp::new(nwk_addr_of_interest, node_descriptor, Vec::new());

        if let Err(error) = self.respond(seq, node_id, payload).await {
            error!("Failed to send Node_Desc_rsp: {error:?}");
        }
    }

    /// Reject a remote management permit-joining request without changing local joining state.
    async fn handle_mgmt_permit_joining_req(
        &self,
        source: NetworkAddress,
        request_was_broadcast: bool,
        seq: u8,
    ) {
        let Some(payload) = permit_joining_response(request_was_broadcast) else {
            return;
        };

        let Ok(node_id) = source.as_u16().try_into().inspect_err(|error| {
            warn!("Invalid node ID: {error:?}");
        }) else {
            return;
        };

        if let Err(error) = self.respond(seq, node_id, payload).await {
            error!("Failed to send Mgmt_Permit_Joining_rsp: {error:?}");
        }
    }
}

fn handle_device_annce(events: &EventSink, device_annce: &DeviceAnnce) {
    let Ok(short_id) = device_annce.nwk_addr().try_into().inspect_err(|error| {
        warn!("Invalid node ID: {error:?}");
    }) else {
        return;
    };

    events.emit(Event::Device(DeviceEvent::Announced(FullAddress::new(
        device_annce.ieee_addr(),
        short_id,
    ))));
}

const fn is_server_request(command: &Command) -> bool {
    matches!(
        command,
        Command::DeviceAndServiceDiscovery(
            DeviceAndServiceDiscovery::NwkAddrReq(_)
                | DeviceAndServiceDiscovery::IeeeAddrReq(_)
                | DeviceAndServiceDiscovery::PowerDescReq(_)
                | DeviceAndServiceDiscovery::SimpleDescReq(_)
                | DeviceAndServiceDiscovery::ActiveEpReq(_)
                | DeviceAndServiceDiscovery::MatchDescReq(_)
                | DeviceAndServiceDiscovery::NodeDescReq(_)
                | DeviceAndServiceDiscovery::SystemServerDiscoveryReq(_)
        ) | Command::NetworkManagement(NetworkManagement::MgmtPermitJoiningReq(_))
    )
}

/// Validate an indication's ZDP addressing and derive its response-correlation key.
fn received_key<T, K>(
    indication: &DataIndication<Frame<Command>, T, K>,
) -> Option<(NetworkAddress, Key)> {
    let source = indication.metadata().source();
    let Some(source_address) = source.network_address() else {
        warn!("Discarding ZDP indication from non-network source: {source:?}");
        return None;
    };
    let Some(key) = Key::from_received_zdp_indication(indication) else {
        warn!(
            "Discarding ZDP indication not addressed between endpoint zero: source={source:?} destination={:?}",
            indication.metadata().destination()
        );
        return None;
    };

    Some((source_address, key))
}

/// Await a deferred ZDP response transmission and report any failure through the actor inbox.
fn track_reply_completion(transmission: TransmissionResponse, inbox: WeakSender<Message>) {
    spawn(async move {
        let Err(error) = transmission.await else {
            return;
        };
        let Some(inbox) = inbox.upgrade() else {
            error!("ZDP server response transmission failed after actor shutdown: {error}");
            return;
        };
        if let Err(send_error) = inbox.send(Message::ReplyTransmissionFailed { error }).await {
            let Message::ReplyTransmissionFailed { error } = send_error.0 else {
                unreachable!("the failed message remains a ZDP reply transmission failure");
            };
            log::error!("Failed to report ZDP server response transmission failure: {error}");
        }
    });
}

const fn permit_joining_response(request_was_broadcast: bool) -> Option<MgmtPermitJoiningRsp> {
    if request_was_broadcast {
        None
    } else {
        Some(MgmtPermitJoiningRsp::new(Status::InvalidRequestType))
    }
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroUsize;
    use std::sync::Mutex;
    use std::time::Duration;

    use bytes::Bytes;
    use le_stream::ToLeStream;
    use tokio::runtime::Runtime;
    use tokio::sync::mpsc::channel;
    use tokio::sync::oneshot;
    use tokio::time::timeout;
    use zb_aps::apsde::{
        ConfirmStatus, DataIndication, IndicationMetadata, IndicationStatus, IndividualEndpoint,
        NetworkAddress, ReceivedDestination, Security, Source, Status as ApsStatus,
    };
    use zb_core::node::Descriptor;
    use zb_core::short_id::Device;
    use zb_core::{ClusterSpecific, Destination, Endpoint, IeeeAddress, Profile, destination};
    use zb_hw::{
        ChannelMask, Driver, Error as HardwareError, FoundNetwork, Operation, ScanDuration,
        ScannedChannel,
    };
    use zb_zdp::{
        ActiveEpReq, Command, DeviceAndServiceDiscovery, Frame, SimpleDescriptor, Status,
    };

    use super::{Message, Transceiver, permit_joining_response, track_reply_completion};
    use crate::Error;
    use crate::aps::{Aps, Message as ApsMessage, Metadata, TransmissionResponse};
    use crate::correlation::Key;
    use crate::event::EventSink;

    const CHANNEL_SIZE: usize = 1;
    const APS_COUNTER: u8 = 1;
    const LINK_QUALITY: u8 = u8::MAX;
    const LOCAL_ADDRESS: u16 = 0;
    const REMOTE_ADDRESS: u16 = 1;
    const SEQUENCE: u8 = 42;
    const NCP_CHANNEL_SIZE: NonZeroUsize = NonZeroUsize::MIN;
    const TEST_TIMEOUT: Duration = Duration::from_millis(100);

    #[derive(Debug)]
    struct DelayedEndpointDriver {
        started: Mutex<Option<oneshot::Sender<()>>>,
        release: Mutex<Option<oneshot::Receiver<()>>>,
    }

    impl Driver for DelayedEndpointDriver {
        async fn get_endpoints(&self) -> Result<Box<[SimpleDescriptor]>, HardwareError> {
            self.started
                .lock()
                .expect("test driver start lock is available")
                .take()
                .expect("test driver receives one endpoint query")
                .send(())
                .expect("test waits for the endpoint query");
            let release = self
                .release
                .lock()
                .expect("test driver release lock is available")
                .take()
                .expect("test driver receives one endpoint query");
            release
                .await
                .expect("test controls endpoint query completion");
            Ok(Vec::new().into_boxed_slice())
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
            _short_id: Device,
        ) -> Result<IeeeAddress, HardwareError> {
            unsupported(Operation::ShortIdToIeeeAddress)
        }

        async fn ieee_address_to_short_id(
            &mut self,
            _ieee_address: IeeeAddress,
        ) -> Result<Device, HardwareError> {
            unsupported(Operation::IeeeAddressToShortId)
        }

        async fn transmit(
            &mut self,
            _request: zb_aps::apsde::DataRequest<Bytes>,
            _counter: u8,
        ) -> Result<(), HardwareError> {
            unsupported(Operation::Transmit)
        }
    }

    #[test]
    fn permit_joining_rejects_unicast_and_ignores_broadcast_requests() {
        let response =
            permit_joining_response(false).expect("a unicast request requires a rejection");

        assert_eq!(response.status(), Ok(Status::InvalidRequestType));
        assert!(permit_joining_response(true).is_none());
    }

    #[test]
    fn reports_deferred_backend_rejection_through_the_actor_inbox() {
        Runtime::new()
            .expect("runtime must be available")
            .block_on(async {
                let message =
                    tracked_reply_failure(zb_hw::TransmissionError::Rejected.into()).await;

                assert!(matches!(
                    message,
                    Message::ReplyTransmissionFailed {
                        error: zb_hw::Error::Transmission(zb_hw::TransmissionError::Rejected)
                    }
                ));
            });
    }

    #[test]
    fn reports_deferred_acknowledgement_failure_through_the_actor_inbox() {
        Runtime::new()
            .expect("runtime must be available")
            .block_on(async {
                let status = ConfirmStatus::Aps(ApsStatus::NoAcknowledgement);
                let message =
                    tracked_reply_failure(zb_hw::TransmissionError::Confirmation(status).into())
                        .await;

                assert!(matches!(
                    message,
                    Message::ReplyTransmissionFailed {
                        error: zb_hw::Error::Transmission(
                            zb_hw::TransmissionError::Confirmation(received)
                        )
                    } if received == status
                ));
            });
    }

    #[test]
    fn actor_handles_hardware_shutdown_while_endpoint_query_is_pending() {
        Runtime::new()
            .expect("runtime must be available")
            .block_on(async {
                let (started, query_started) = oneshot::channel();
                let (release_query, release) = oneshot::channel();
                let (ncp, driver) = DelayedEndpointDriver {
                    started: Mutex::new(Some(started)),
                    release: Mutex::new(Some(release)),
                }
                .into_actor(NCP_CHANNEL_SIZE);
                let driver = tokio::spawn(driver);
                let (aps_messages, _aps_receiver) = channel(CHANNEL_SIZE);
                let (events, _event_receiver) = channel(CHANNEL_SIZE);
                let zdp = Transceiver::spawn(
                    ncp,
                    Aps::new(aps_messages),
                    EventSink::new(events),
                    Descriptor::default(),
                );

                zdp.send(Message::Received {
                    indication: active_endpoint_request(),
                })
                .await
                .expect("ZDP actor accepts the request");
                timeout(TEST_TIMEOUT, query_started)
                    .await
                    .expect("endpoint query must start")
                    .expect("endpoint query start sender remains available");
                zdp.send(Message::HardwareUnavailable)
                    .await
                    .expect("ZDP actor accepts hardware shutdown");
                timeout(TEST_TIMEOUT, zdp.closed())
                    .await
                    .expect("pending endpoint query must not block actor shutdown");

                release_query
                    .send(())
                    .expect("driver still waits for endpoint query completion");
                drop(zdp);
                timeout(TEST_TIMEOUT, driver)
                    .await
                    .expect("driver actor must stop after ZDP shutdown")
                    .expect("driver actor task must not panic");
            });
    }

    #[test]
    fn network_down_quarantines_a_submission_that_completed_outside_the_actor() {
        Runtime::new()
            .expect("runtime must be available")
            .block_on(async {
                let (started, _query_started) = oneshot::channel();
                let (_release_query, release) = oneshot::channel();
                let (ncp, driver) = DelayedEndpointDriver {
                    started: Mutex::new(Some(started)),
                    release: Mutex::new(Some(release)),
                }
                .into_actor(NCP_CHANNEL_SIZE);
                let driver = tokio::spawn(driver);
                let (aps_inbox, mut aps_messages) = channel(CHANNEL_SIZE);
                let (events, _event_receiver) = channel(CHANNEL_SIZE);
                let (zdp_inbox, mut zdp_messages) = channel(CHANNEL_SIZE);
                let mut transceiver = Transceiver::new(
                    ncp,
                    Aps::new(aps_inbox.clone()),
                    EventSink::new(events),
                    Descriptor::default(),
                    zdp_inbox.downgrade(),
                );
                let device = Device::new(REMOTE_ADDRESS).expect("test device ID is valid");
                let request = communication_request(device);
                let (response, result) = oneshot::channel();

                transceiver.communicate(device, request, response);
                let original_sequence = transceiver
                    .communication_submissions
                    .values()
                    .next()
                    .expect("communication submission is tracked")
                    .token
                    .key()
                    .sequence();
                let ApsMessage::Transmit { response, .. } = aps_messages
                    .recv()
                    .await
                    .expect("submission reaches the APS actor")
                else {
                    panic!("expected APS transmission");
                };
                let (_completion, deferred) = oneshot::channel();
                let transmission =
                    TransmissionResponse::test_new(deferred, APS_COUNTER, aps_inbox.downgrade());
                response
                    .send(Ok(transmission))
                    .expect("submission task still waits for APS handoff");
                let stale_completion = timeout(TEST_TIMEOUT, zdp_messages.recv())
                    .await
                    .expect("submission completion must be queued")
                    .expect("ZDP actor inbox remains available");

                assert!(transceiver.handle_actor_message(Message::NetworkDown));
                assert!(matches!(
                    result.await,
                    Ok(Err(Error::Hardware(HardwareError::Transmission(
                        zb_hw::TransmissionError::NoRoute
                    ))))
                ));
                assert!(transceiver.handle_actor_message(stale_completion));

                let request = communication_request(device);
                let (next_sequence, token, _response) = transceiver
                    .responses
                    .register(|sequence| Key::from_zdp_command(device, sequence, &request))
                    .expect("another transaction sequence remains available");
                assert_ne!(next_sequence, original_sequence);
                transceiver.responses.discard(token);

                drop(transceiver);
                drop(aps_inbox);
                drop(zdp_inbox);
                timeout(TEST_TIMEOUT, driver)
                    .await
                    .expect("driver actor must stop after transceiver shutdown")
                    .expect("driver actor task must not panic");
            });
    }

    async fn tracked_reply_failure(error: zb_hw::Error) -> Message {
        let (aps_inbox, _aps_messages) = channel::<ApsMessage>(CHANNEL_SIZE);
        let (completion, result) = oneshot::channel();
        let transmission =
            TransmissionResponse::test_new(result, APS_COUNTER, aps_inbox.downgrade());
        let (zdp_inbox, mut zdp_messages) = channel(CHANNEL_SIZE);
        track_reply_completion(transmission, zdp_inbox.downgrade());

        completion
            .send(Err(error))
            .expect("transmission response must be waiting");
        zdp_messages
            .recv()
            .await
            .expect("deferred failure must reach the ZDP actor inbox")
    }

    fn active_endpoint_request() -> DataIndication<Frame<Command>, (), ()> {
        let command: Command =
            DeviceAndServiceDiscovery::from(ActiveEpReq::new(LOCAL_ADDRESS)).into();
        let metadata = IndicationMetadata::new(
            ReceivedDestination::Network {
                address: network_address(LOCAL_ADDRESS),
                endpoint: data_endpoint(),
            },
            Source::Network {
                address: network_address(REMOTE_ADDRESS),
                endpoint: data_endpoint(),
            },
            Profile::Network.as_u16(),
            command.cluster_id(),
            IndicationStatus::success(),
            Security::<()>::Unsecured,
            LINK_QUALITY,
            (),
        );

        DataIndication::new(metadata, Frame::new(SEQUENCE, command))
    }

    fn communication_request(device: Device) -> zb_aps::apsde::DataRequest<Bytes> {
        crate::aps::data_request(
            Destination::Device(destination::Device::new(device, Endpoint::Data)),
            Endpoint::Data,
            Metadata::new(Profile::Network, <ActiveEpReq as ClusterSpecific>::ID),
            ActiveEpReq::new(REMOTE_ADDRESS).to_le_stream().collect(),
        )
    }

    fn data_endpoint() -> IndividualEndpoint {
        IndividualEndpoint::new(Endpoint::Data).expect("data endpoint is individual")
    }

    fn network_address(address: u16) -> NetworkAddress {
        NetworkAddress::new(address).expect("test NWK address is valid")
    }

    fn unsupported<T>(operation: Operation) -> Result<T, HardwareError> {
        Err(HardwareError::Unsupported(operation))
    }
}
