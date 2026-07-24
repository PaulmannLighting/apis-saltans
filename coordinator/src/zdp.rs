//! Transceiver to send and receive ZDP messages.

use std::collections::BTreeMap;

use le_stream::ToLeStream;
use log::{debug, error, trace, warn};
use tokio::spawn;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::oneshot;
use tokio::sync::oneshot::channel;
use zb_aps::data::Header;
use zb_aps::{Data, DeliveryMode};
use zb_core::node::Descriptor;
use zb_core::short_id::Device;
use zb_core::{Destination, Endpoint, FullAddress, destination};
use zb_hw::Ncp;
use zb_nwk::Source;
use zb_zdp::{
    Command, DeviceAndServiceDiscovery, DeviceAnnce, Frame, MatchDescReq, MatchDescRsp,
    MgmtPermitJoiningRsp, NetworkManagement, NodeDescReq, NodeDescRsp, Status,
};

use self::match_desc::{
    Action as MatchDescAction, action as match_desc_action, matching_endpoints,
};
pub use self::message::Message;
use self::node_desc::{
    Action as NodeDescAction, action as node_desc_action, unavailable_child_status,
};
pub use self::payload::Payload;
use super::index::Index;
use crate::aps::Aps;
use crate::response::InternalCommunicationResponse;
use crate::{Device as DeviceEvent, Event, MPSC_CHANNEL_SIZE};

mod match_desc;
mod message;
mod node_desc;
mod payload;

/// Zigbee transceiver actor.
#[derive(Debug)]
pub struct Transceiver<T> {
    ncp: T,
    aps: Aps,
    events: Sender<Event>,
    descriptor: Descriptor,
    /// Whether the hardware has reported that joining is open.
    joining_permitted: bool,
    responses: BTreeMap<Index, oneshot::Sender<Command>>,
    seq: u8,
}

impl<T> Transceiver<T> {
    /// Create a new transceiver.
    #[must_use]
    pub const fn new(ncp: T, aps: Aps, events: Sender<Event>, descriptor: Descriptor) -> Self {
        Self {
            ncp,
            aps,
            events,
            descriptor,
            joining_permitted: false,
            responses: BTreeMap::new(),
            seq: 0,
        }
    }
}

impl<T> Transceiver<T>
where
    T: Ncp + Sync,
{
    /// Run the transceiver.
    pub async fn run(mut self, mut messages: Receiver<Message>) {
        while let Some(message) = messages.recv().await {
            match message {
                Message::Received { source, frame } => {
                    self.handle_message_received(source, frame).await;
                }
                Message::NetworkOpened => {
                    self.joining_permitted = true;
                }
                Message::NetworkClosed => {
                    self.joining_permitted = false;
                }
                Message::Communicate {
                    device,
                    payload,
                    response,
                } => {
                    response
                        .send(self.communicate(device, payload).await)
                        .unwrap_or_else(|error| {
                            debug!("Failed to send unicast response: {error:?}");
                        });
                }
            }
        }
    }

    /// Return and increment the ZCL sequence number.
    const fn next_seq(&mut self) -> u8 {
        let seq = self.seq;
        self.seq = self.seq.wrapping_add(1);
        seq
    }

    async fn handle_message_received(&mut self, source: Source, frame: Data<Frame<Command>>) {
        trace!("Received ZDP message from {source}: {frame:?}");
        let (aps_header, zdp_frame) = frame.into_parts();
        let index = Index::from_received_zdp_frame(source, &zdp_frame);
        let (seq, command) = zdp_frame.into_parts();

        match command {
            Command::DeviceAndServiceDiscovery(DeviceAndServiceDiscovery::MatchDescReq(
                match_desc_req,
            )) => {
                self.handle_match_desc_req(source, aps_header, seq, *match_desc_req)
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
                self.handle_node_desc_req(source, seq, *node_desc_req).await;
            }
            Command::NetworkManagement(NetworkManagement::MgmtPermitJoiningReq(_)) => {
                self.handle_mgmt_permit_joining_req(source, seq).await;
            }
            command => {
                if let Some(sender) = self.responses.remove(&index) {
                    debug!(
                        "Answering ZDP request: seq={seq} cluster_id={:#06X}",
                        command.cluster_id()
                    );
                    sender.send(command).unwrap_or_else(|error| {
                        warn!("Failed to send ZDP response: {error:?}");
                    });
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
        payload: Payload,
    ) -> Result<InternalCommunicationResponse<Command>, zb_hw::Error> {
        let (metadata, payload) = payload.into_parts();
        let seq = self.next_seq();
        let index = Index::from_zdp_command(device, seq, metadata);
        let zdp_frame = Frame::new(seq, payload);
        let destination = Destination::Device(destination::Device::new(device, Endpoint::Data));
        let payload = zdp_frame.to_le_stream().collect();
        let (tx, rx) = channel();
        self.responses.insert(index, tx);

        if let Err(error) = self.aps.transmit(destination, metadata, payload).await {
            self.responses.remove(&index);
            return Err(error);
        }

        Ok(InternalCommunicationResponse::new(rx))
    }

    async fn respond(&self, seq: u8, device: Device, payload: Payload) -> Result<(), zb_hw::Error> {
        let (metadata, payload) = payload.into_parts();
        let zdp_frame = Frame::new(seq, payload);
        let destination = Destination::Device(destination::Device::new(device, Endpoint::Data));
        self.aps
            .transmit(destination, metadata, zdp_frame.to_le_stream().collect())
            .await
    }

    /// Process a Match Descriptor request and unicast any required response to its originator.
    async fn handle_match_desc_req(
        &self,
        source: Source,
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

        let Ok(node_id) = source.node_id().try_into().inspect_err(|error| {
            warn!("Invalid node ID: {error:?}");
        }) else {
            return;
        };

        if let Err(error) = self.respond(seq, node_id, Payload::from(response)).await {
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
    async fn handle_node_desc_req(&self, source: Source, seq: u8, node_desc_req: NodeDescReq) {
        let Ok(node_id) = source.node_id().try_into().inspect_err(|error| {
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

        if let Err(error) = self.respond(seq, node_id, Payload::from(payload)).await {
            error!("Failed to send Node_Desc_rsp: {error:?}");
        }
    }

    /// Apply a management permit-joining request and return its result to the requester.
    async fn handle_mgmt_permit_joining_req(&self, source: Source, seq: u8) {
        let Ok(node_id) = source.node_id().try_into().inspect_err(|error| {
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

        if let Err(error) = self.respond(seq, node_id, Payload::from(payload)).await {
            error!("Failed to send Mgmt_Permit_Joining_rsp: {error:?}");
        }
    }
}

impl<T> Transceiver<T>
where
    T: Ncp + Send + Sync + 'static,
{
    /// Start the ZDP transceiver.
    pub fn spawn(
        ncp: T,
        aps: Aps,
        events: Sender<Event>,
        descriptor: Descriptor,
    ) -> Sender<Message> {
        let (zdp_tx, zdp_rx) = tokio::sync::mpsc::channel(MPSC_CHANNEL_SIZE);
        spawn(Self::new(ncp, aps, events, descriptor).run(zdp_rx));
        zdp_tx
    }
}
