//! Transceiver to send and receive ZDP messages.

use std::collections::BTreeMap;

use le_stream::ToLeStream;
use log::{debug, error, trace, warn};
use tokio::spawn;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::oneshot;
use tokio::sync::oneshot::channel;
use zb_aps::Data;
use zb_core::node::Descriptor;
use zb_core::short_id::{Device, ShortId};
use zb_core::{Destination, Endpoint, FullAddress, destination};
use zb_hw::{Datagram, Ncp};
use zb_nwk::Source;
use zb_zdp::{
    Command, DeviceAndServiceDiscovery, DeviceAnnce, Frame, MatchDescReq, MatchDescRsp,
    MgmtPermitJoiningRsp, NetworkManagement, NodeDescReq, NodeDescRsp, Status,
};

use self::match_desc_req_ext::MatchDescReqExt;
pub use self::message::Message;
pub use self::payload::Payload;
use super::index::Index;
use crate::response::InternalCommunicationResponse;
use crate::{Device as DeviceEvent, Event, MPSC_CHANNEL_SIZE};

mod match_desc_req_ext;
mod message;
mod payload;

/// Zigbee transceiver actor.
#[derive(Debug)]
pub struct Transceiver<T> {
    ncp: T,
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
    pub const fn new(ncp: T, events: Sender<Event>, descriptor: Descriptor) -> Self {
        Self {
            ncp,
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
        let (_, zdp_frame) = frame.into_parts();
        let index = Index::from_received_zdp_frame(source, &zdp_frame);
        let (seq, command) = zdp_frame.into_parts();

        if let Command::DeviceAndServiceDiscovery(DeviceAndServiceDiscovery::MatchDescReq(
            match_desc_req,
        )) = command
        {
            self.handle_match_desc_req(source, seq, *match_desc_req)
                .await;
            return;
        }

        if let Command::DeviceAndServiceDiscovery(DeviceAndServiceDiscovery::DeviceAnnce(
            device_annce,
        )) = command
        {
            self.handle_device_annce(*device_annce).await;
            return;
        }

        if let Command::DeviceAndServiceDiscovery(DeviceAndServiceDiscovery::NodeDescReq(
            node_desc_req,
        )) = command
        {
            self.handle_node_desc_req(source, seq, *node_desc_req).await;
            return;
        }

        if let Command::NetworkManagement(NetworkManagement::MgmtPermitJoiningReq(_)) = command {
            self.handle_mgmt_permit_joining_req(source, seq).await;
            return;
        }

        if let Some(sender) = self.responses.remove(&index) {
            debug!(
                "Answering ZDP request: seq={seq} cluster_id={:#06X}",
                command.cluster_id()
            );
            sender.send(command).unwrap_or_else(|error| {
                warn!("Failed to send ZDP response: {error:?}");
            });
            return;
        }

        warn!("Unexpected ZDP response: {command:?}");
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
        #[expect(unsafe_code)]
        // SAFETY: We construct the datagram from the unchanged metadata and correct ZDP payload.
        let hw_datagram =
            unsafe { Datagram::new_unchecked(metadata, zdp_frame.to_le_stream().collect()) };
        let (tx, rx) = channel();
        self.responses.insert(index, tx);
        let transmission_rx = self
            .ncp
            .transmit(
                Destination::Device(destination::Device::new(device, Endpoint::Data)),
                hw_datagram,
            )
            .await?;
        Ok(InternalCommunicationResponse::new(transmission_rx, rx))
    }

    async fn respond(&self, seq: u8, device: Device, payload: Payload) -> Result<(), zb_hw::Error> {
        let (metadata, payload) = payload.into_parts();
        let zdp_frame = Frame::new(seq, payload);
        #[expect(unsafe_code)]
        // SAFETY: We construct the datagram from the unchanged metadata and correct ZDP payload.
        let hw_datagram =
            unsafe { Datagram::new_unchecked(metadata, zdp_frame.to_le_stream().collect()) };
        self.ncp
            .transmit(
                Destination::Device(destination::Device::new(device, Endpoint::Data)),
                hw_datagram,
            )
            .await?
            .await?;
        Ok(())
    }

    async fn handle_match_desc_req(&self, source: Source, seq: u8, match_desc_req: MatchDescReq) {
        let Ok(endpoints) = self.ncp.get_endpoints().await else {
            return;
        };

        let payload = MatchDescRsp::new(
            match_desc_req.nwk_addr_of_interest(),
            Ok(endpoints
                .iter()
                .filter_map(|endpoint| {
                    if match_desc_req.matches(endpoint) {
                        Some(endpoint.endpoint_id())
                    } else {
                        None
                    }
                })
                .collect()),
        );

        let Ok(node_id) = source.node_id().try_into().inspect_err(|error| {
            warn!("Invalid node ID: {error:?}");
        }) else {
            return;
        };

        if let Err(error) = self.respond(seq, node_id, Payload::from(payload)).await {
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

    /// Respond to a node-descriptor request addressed to this coordinator.
    async fn handle_node_desc_req(&self, source: Source, seq: u8, node_desc_req: NodeDescReq) {
        let coordinator_address = ShortId::Coordinator.as_u16();

        if node_desc_req.nwk_addr() != coordinator_address {
            return;
        }

        let Ok(node_id) = source.node_id().try_into().inspect_err(|error| {
            warn!("Invalid node ID: {error:?}");
        }) else {
            return;
        };

        let payload =
            NodeDescRsp::new(coordinator_address, Ok(self.descriptor.clone()), Vec::new());

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
    pub fn spawn(ncp: T, events: Sender<Event>, descriptor: Descriptor) -> Sender<Message> {
        let (zdp_tx, zdp_rx) = tokio::sync::mpsc::channel(MPSC_CHANNEL_SIZE);
        spawn(Self::new(ncp, events, descriptor).run(zdp_rx));
        zdp_tx
    }
}
