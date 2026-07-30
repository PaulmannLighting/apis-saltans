//! Transceiver to send and receive ZDP messages.

use bytes::Bytes;
use le_stream::ToLeStream;
use log::{debug, error, trace, warn};
use tokio::runtime::Handle;
use tokio::spawn;
use tokio::sync::mpsc::error::TrySendError;
use tokio::sync::mpsc::{Receiver, Sender, WeakSender};
use tokio::time::sleep;
use zb_aps::DeliveryMode;
use zb_aps::apsde::{DataIndication, DataRequest, NetworkAddress};
use zb_aps::data::Header;
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
    Action as MatchDescAction, action as match_desc_action, matching_endpoints,
};
pub use self::message::Message;
use self::node_desc::{
    Action as NodeDescAction, action as node_desc_action, unavailable_child_status,
};
use super::index::Index;
use crate::aps::{Aps, Metadata};
use crate::correlation::{Cancellation, PROTOCOL_RESPONSE_TIMEOUT, Registry};
use crate::response::ApsProtocolResponse;
use crate::{Device as DeviceEvent, Event, MPSC_CHANNEL_SIZE};

mod discovery;
mod match_desc;
mod message;
mod node_desc;

/// Zigbee transceiver actor.
#[derive(Debug)]
pub struct Transceiver {
    ncp: NcpHandle,
    aps: Aps,
    events: Sender<Event>,
    descriptor: Descriptor,
    /// Whether the hardware has reported that joining is open.
    joining_permitted: bool,
    responses: Registry<Command>,
    inbox: WeakSender<Message>,
}

impl Transceiver {
    /// Create a new transceiver.
    #[must_use]
    pub const fn new(
        ncp: NcpHandle,
        aps: Aps,
        events: Sender<Event>,
        descriptor: Descriptor,
        inbox: WeakSender<Message>,
    ) -> Self {
        Self {
            ncp,
            aps,
            events,
            descriptor,
            joining_permitted: false,
            responses: Registry::new(),
            inbox,
        }
    }

    /// Run the transceiver.
    pub async fn run(mut self, mut messages: Receiver<Message>) {
        while let Some(message) = messages.recv().await {
            self.handle_actor_message(message).await;
        }
    }

    async fn handle_actor_message(&mut self, message: Message) {
        match message {
            Message::Received { indication } => {
                self.handle_message_received(indication).await;
            }
            Message::NetworkOpened => {
                self.joining_permitted = true;
            }
            Message::NetworkClosed => {
                self.joining_permitted = false;
            }
            Message::NetworkDown => {
                self.responses.fail_all(&zb_hw::TransmissionError::NoRoute);
            }
            Message::Cancel { index } => {
                self.responses.cancel(index);
            }
            Message::ResponseTimeout { index } => {
                self.responses.timeout(index);
            }
            Message::Communicate {
                device,
                request,
                response,
            } => {
                response
                    .send(self.communicate(device, request).await)
                    .unwrap_or_else(|error| {
                        debug!("Failed to send unicast response: {error:?}");
                    });
            }
        }
    }

    async fn handle_message_received(
        &mut self,
        indication: DataIndication<Frame<Command>, (), ()>,
    ) {
        let Some((source, frame)) = crate::apsde::into_legacy_data(indication) else {
            warn!("Discarding ZDP indication with unsupported addressing");
            return;
        };
        let Some(source_address) = source.network_address() else {
            warn!("Discarding ZDP indication from non-network source: {source:?}");
            return;
        };
        trace!("Received ZDP message from {source:?}: {frame:?}");
        let (aps_header, zdp_frame) = frame.into_parts();
        let index = Index::from_received_zdp_frame(source_address, &zdp_frame);
        let (seq, command) = zdp_frame.into_parts();

        match command {
            Command::DeviceAndServiceDiscovery(DeviceAndServiceDiscovery::NwkAddrReq(
                nwk_addr_req,
            )) => {
                self.handle_nwk_addr_req(source_address, seq, *nwk_addr_req)
                    .await;
            }
            Command::DeviceAndServiceDiscovery(DeviceAndServiceDiscovery::IeeeAddrReq(
                ieee_addr_req,
            )) => {
                self.handle_ieee_addr_req(source_address, seq, *ieee_addr_req)
                    .await;
            }
            Command::DeviceAndServiceDiscovery(DeviceAndServiceDiscovery::PowerDescReq(
                power_desc_req,
            )) => {
                self.handle_power_desc_req(source_address, seq, *power_desc_req)
                    .await;
            }
            Command::DeviceAndServiceDiscovery(DeviceAndServiceDiscovery::SimpleDescReq(
                simple_desc_req,
            )) => {
                self.handle_simple_desc_req(source_address, seq, *simple_desc_req)
                    .await;
            }
            Command::DeviceAndServiceDiscovery(DeviceAndServiceDiscovery::ActiveEpReq(
                active_ep_req,
            )) => {
                self.handle_active_ep_req(source_address, seq, *active_ep_req)
                    .await;
            }
            Command::DeviceAndServiceDiscovery(DeviceAndServiceDiscovery::MatchDescReq(
                match_desc_req,
            )) => {
                self.handle_match_desc_req(source_address, aps_header, seq, *match_desc_req)
                    .await;
            }
            Command::DeviceAndServiceDiscovery(DeviceAndServiceDiscovery::DeviceAnnce(
                device_annce,
            )) => {
                self.handle_device_annce(*device_annce).await;
            }
            Command::DeviceAndServiceDiscovery(DeviceAndServiceDiscovery::NodeDescReq(
                node_desc_req,
            )) => {
                self.handle_node_desc_req(source_address, seq, *node_desc_req)
                    .await;
            }
            Command::DeviceAndServiceDiscovery(
                DeviceAndServiceDiscovery::SystemServerDiscoveryReq(system_server_discovery_req),
            ) => {
                self.handle_system_server_discovery_req(
                    source_address,
                    seq,
                    *system_server_discovery_req,
                )
                .await;
            }
            Command::NetworkManagement(NetworkManagement::MgmtPermitJoiningReq(_)) => {
                self.handle_mgmt_permit_joining_req(source_address, seq)
                    .await;
            }
            command => {
                if self.responses.complete(index, command.clone()) {
                    debug!(
                        "Answering ZDP request: seq={seq} cluster_id={:#06X}",
                        command.cluster_id()
                    );
                } else if self.responses.is_quarantined(index) {
                    debug!("Discarding late ZDP response with quarantined sequence {seq}");
                } else {
                    warn!("Unexpected ZDP response: {command:?}");
                }
            }
        }
    }

    /// Send a ZDP unicast message with back-channel communication.
    ///
    /// # Returns
    ///
    /// Returns the response receiver.
    ///
    /// # Errors
    ///
    /// Returns an error if the unicast message could not be sent.
    async fn communicate(
        &mut self,
        device: Device,
        request: DataRequest<Bytes>,
    ) -> Result<ApsProtocolResponse<Command>, crate::Error> {
        let (seq, index, rx) = self
            .responses
            .register(|sequence| Index::from_zdp_command(device, sequence, &request))?;
        self.schedule_response_timeout(index);
        let request = request.map_asdu(|payload| Frame::new(seq, payload).to_le_stream().collect());

        let transmission = match self.aps.transmit(request).await {
            Ok(transmission) => transmission,
            Err(error) => {
                self.responses.cancel(index);
                return Err(error);
            }
        };
        let cancellation = self.cancellation(index);

        Ok(ApsProtocolResponse::new(transmission, rx, cancellation))
    }

    fn cancellation(&self, index: Index) -> Cancellation {
        let inbox = self.inbox.clone();
        let runtime = Handle::current();
        Cancellation::new(index, move |index| {
            let Some(inbox) = inbox.upgrade() else {
                return;
            };
            match inbox.try_send(Message::Cancel { index }) {
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

    fn schedule_response_timeout(&self, index: Index) {
        let inbox = self.inbox.clone();
        spawn(async move {
            sleep(PROTOCOL_RESPONSE_TIMEOUT).await;
            let Some(inbox) = inbox.upgrade() else {
                return;
            };
            inbox
                .send(Message::ResponseTimeout { index })
                .await
                .unwrap_or_else(|error| {
                    debug!("Failed to enqueue ZDP response timeout: {error}");
                });
        });
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
        self.aps.transmit(request).await.map(drop)
    }

    /// Process a Match Descriptor request and unicast any required response to its originator.
    async fn handle_match_desc_req(
        &self,
        source: NetworkAddress,
        aps_header: Header,
        seq: u8,
        match_desc_req: MatchDescReq,
    ) {
        let request_was_broadcast = matches!(
            aps_header.control().delivery_mode(),
            Some(DeliveryMode::Broadcast)
        );

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

                    if matches.is_empty() && request_was_broadcast {
                        return;
                    }

                    MatchDescRsp::new(nwk_addr_of_interest, Ok(matches))
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

    async fn handle_device_annce(&self, device_annce: DeviceAnnce) {
        let Ok(short_id) = device_annce.nwk_addr().try_into().inspect_err(|error| {
            warn!("Invalid node ID: {error:?}");
        }) else {
            return;
        };

        self.events
            .send(Event::Device(DeviceEvent::Announced(FullAddress::new(
                device_annce.ieee_addr(),
                short_id,
            ))))
            .await
            .unwrap_or_else(|error| {
                error!("Failed to send device announcement: {error:?}");
            });
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

    /// Apply a management permit-joining request and return its result to the requester.
    async fn handle_mgmt_permit_joining_req(&self, source: NetworkAddress, seq: u8) {
        let Ok(node_id) = source.as_u16().try_into().inspect_err(|error| {
            warn!("Invalid node ID: {error:?}");
        }) else {
            return;
        };

        let status = if self.joining_permitted {
            Status::Success
        } else {
            Status::NotPermitted
        };
        let payload = MgmtPermitJoiningRsp::new(status);

        if let Err(error) = self.respond(seq, node_id, payload).await {
            error!("Failed to send Mgmt_Permit_Joining_rsp: {error:?}");
        }
    }

    /// Start the ZDP transceiver.
    pub fn spawn(
        ncp: NcpHandle,
        aps: Aps,
        events: Sender<Event>,
        descriptor: Descriptor,
    ) -> Sender<Message> {
        let (zdp_tx, zdp_rx) = tokio::sync::mpsc::channel(MPSC_CHANNEL_SIZE);
        spawn(Self::new(ncp, aps, events, descriptor, zdp_tx.downgrade()).run(zdp_rx));
        zdp_tx
    }
}
