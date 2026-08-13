//! State used while serving incoming ZDP requests.

use le_stream::ToLeStream;
use log::{error, warn};
use tokio::spawn;
use tokio::sync::mpsc::WeakSender;
use zb_aps::apsde::{IndividualEndpoint, NetworkAddress, NetworkDestination, RequestDestination};
use zb_core::node::Descriptor;
use zb_core::short_id::Device;
use zb_core::{ClusterSpecific, Endpoint, IeeeAddress, Profile};
use zb_hw::NcpHandle;
use zb_zdp::{
    ActiveEpReq, ActiveEpRsp, Command, DeviceAndServiceDiscovery, Frame, IeeeAddrReq, IeeeAddrRsp,
    IeeeAddrRspResponse, MatchDescReq, MatchDescRsp, MgmtPermitJoiningRsp, NetworkManagement,
    NodeDescReq, NodeDescRsp, NwkAddrReq, NwkAddrRsp, NwkAddrRspResponse, PowerDescReq,
    PowerDescRsp, RequestType, SimpleDescReq, SimpleDescRsp, Status, SystemServerDiscoveryReq,
    SystemServerDiscoveryRsp,
};

use super::Message;
use super::discovery::{
    DescriptorTarget, LOCAL_NWK_ADDRESS, active_endpoints, descriptor_target, matching_server_mask,
    simple_descriptor,
};
use super::match_desc::{
    Action as MatchDescAction, action as match_desc_action, local_response as local_match_response,
    matching_endpoints,
};
use super::node_desc::{
    Action as NodeDescAction, action as node_desc_action, unavailable_child_status,
};
use crate::aps::{Aps, Metadata, TransmissionResponse};

/// Cloneable context used by bounded background ZDP request-serving operations.
#[derive(Clone, Debug)]
pub(super) struct Server {
    ncp: NcpHandle,
    aps: Aps,
    descriptor: Descriptor,
    inbox: WeakSender<Message>,
}

/// A received ZDP request that may require asynchronous NCP or APS work.
#[derive(Debug)]
pub(super) struct ServerRequest {
    source: NetworkAddress,
    request_was_broadcast: bool,
    sequence: u8,
    command: Command,
}

/// Construction, APS access, and dispatch of incoming ZDP requests.
impl Server {
    /// Create a server context for background request handling.
    pub(super) const fn new(
        ncp: NcpHandle,
        aps: Aps,
        descriptor: Descriptor,
        inbox: WeakSender<Message>,
    ) -> Self {
        Self {
            ncp,
            aps,
            descriptor,
            inbox,
        }
    }

    /// Return the APS transceiver used by the server.
    pub(super) const fn aps(&self) -> &Aps {
        &self.aps
    }

    /// Dispatch a received request to its command-specific handler.
    pub(super) async fn handle(&self, request: ServerRequest) {
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
                Ok(descriptors) => request
                    .endpoint()
                    .map_err(|_| Status::InvalidEndpoint)
                    .and_then(|endpoint| simple_descriptor(endpoint, &descriptors)),
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
        let destination: RequestDestination = NetworkDestination::new(
            NetworkAddress::new(device.as_u16())
                .expect("device short addresses are valid APSDE network addresses"),
            IndividualEndpoint::new(Endpoint::Data).expect("ZDO endpoint is individual"),
        )
        .into();
        let request = crate::aps::data_request(
            destination,
            IndividualEndpoint::new(Endpoint::Data).expect("ZDO endpoint is individual"),
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

/// Construction of requests for background server operations.
impl ServerRequest {
    /// Create a request from a received ZDP command and its routing metadata.
    pub(super) const fn new(
        source: NetworkAddress,
        request_was_broadcast: bool,
        sequence: u8,
        command: Command,
    ) -> Self {
        Self {
            source,
            request_was_broadcast,
            sequence,
            command,
        }
    }
}

/// Return whether a command is handled by the local ZDP server.
pub(super) const fn is_server_request(command: &Command) -> bool {
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

/// Await a deferred ZDP response transmission and report any failure through the actor inbox.
pub(super) fn track_reply_completion(
    transmission: TransmissionResponse,
    inbox: WeakSender<Message>,
) {
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

pub(super) const fn permit_joining_response(
    request_was_broadcast: bool,
) -> Option<MgmtPermitJoiningRsp> {
    if request_was_broadcast {
        None
    } else {
        Some(MgmtPermitJoiningRsp::new(Status::InvalidRequestType))
    }
}
