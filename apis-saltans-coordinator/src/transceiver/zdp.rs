//! Transceiver to send and receive ZDP messages.

use std::collections::BTreeMap;

use apis_saltans_core::{Address, Endpoint};
use apis_saltans_hw::{Metadata, Ncp};
use apis_saltans_zdp::{
    Command, DeviceAndServiceDiscovery, DeviceAnnce, Frame, MatchDescReq, MatchDescRsp,
    SimpleDescriptor, Status,
};
use le_stream::ToLeStream;
use log::{debug, error, trace, warn};
use tokio::spawn;
use tokio::sync::mpsc::{Receiver, WeakSender};
use tokio::sync::oneshot;
use tokio::sync::oneshot::{Sender, channel};

pub use self::handle::Handle;
use self::match_desc_req_ext::MatchDescReqExt;
pub use self::message::{Message, Payload};
use crate::{MPSC_CHANNEL_SIZE, discovery};

mod handle;
mod match_desc_req_ext;
mod message;

const CLUSTER_ID_RESPONSE_MASK: u16 = 0x8000;

/// Zigbee transceiver actor.
#[derive(Debug)]
pub struct Transceiver<T> {
    ncp: T,
    discovery: WeakSender<discovery::Message>,
    endpoints: Box<[SimpleDescriptor]>,
    responses: BTreeMap<(u8, u16), Sender<Command>>,
    seq: u8,
}

impl<T> Transceiver<T> {
    /// Create a new transceiver.
    #[must_use]
    pub const fn new(
        ncp: T,
        discovery: WeakSender<discovery::Message>,
        endpoints: Box<[SimpleDescriptor]>,
    ) -> Self {
        Self {
            ncp,
            discovery,
            endpoints,
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
                Message::Received { src_address, frame } => {
                    self.handle_message_received(src_address, *frame).await;
                }
                Message::Communicate {
                    short_id,
                    command: payload,
                    response,
                } => {
                    response
                        .send(self.communicate(short_id, *payload).await)
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

    async fn handle_message_received(&mut self, src_address: u16, frame: Frame<Command>) {
        trace!("Received ZDP message from {src_address}: {frame:?}");
        let (seq, command) = frame.into_parts();

        if let Command::DeviceAndServiceDiscovery(DeviceAndServiceDiscovery::MatchDescReq(
            match_desc_req,
        )) = command
        {
            self.handle_match_desc_req(src_address, seq, *match_desc_req)
                .await;
            return;
        }

        if let Command::DeviceAndServiceDiscovery(DeviceAndServiceDiscovery::DeviceAnnce(
            device_annce,
        )) = command
        {
            self.handle_device_annce(device_annce).await;
            return;
        }

        if let Some(sender) = self.responses.remove(&(seq, command.cluster_id())) {
            debug!(
                "Answering ZDP request: seq={seq} cluster_id={:#06X}",
                command.cluster_id()
            );
            sender.send(command).unwrap_or_else(|error| {
                warn!("Failed to send ZDP response: {error:?}");
            });
        }
    }

    /// Send a ZDP unicast message.
    ///
    /// # Returns
    ///
    /// Returns the ZDP sequence number.
    ///
    /// # Errors
    ///
    /// Returns an error if the unicast message could not be sent.
    async fn unicast(
        &self,
        seq: u8,
        short_id: u16,
        payload: Payload<Command>,
    ) -> Result<(), apis_saltans_hw::Error> {
        let (metadata, command) = payload.into_parts();
        let zdp_frame = Frame::new(seq, command);

        #[expect(unsafe_code)]
        // SAFETY: We extracted the metadata and command from the payload above
        // and created a valid ZDP frame from that command.
        // Hence, the resulting metadata and payload match.
        let aps_frame = unsafe { Self::make_aps_frame(metadata, zdp_frame) };

        debug!("Sending ZDP message: seq={seq}");
        self.ncp
            .unicast(short_id, Endpoint::Data, aps_frame)
            .await
            .map(drop)
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
        short_id: u16,
        command: Command,
    ) -> Result<oneshot::Receiver<Command>, apis_saltans_hw::Error> {
        let seq = self.next_seq();
        let cluster_id = command.cluster_id();
        self.unicast(seq, short_id, command.into()).await?;
        let (tx, rx) = channel();
        self.responses
            .insert((seq, cluster_id | CLUSTER_ID_RESPONSE_MASK), tx);
        Ok(rx)
    }

    /// Create a new APS frame.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the given metadata and payload match.
    #[expect(unsafe_code)]
    unsafe fn make_aps_frame(metadata: Metadata, frame: Frame<Command>) -> apis_saltans_hw::Frame {
        let payload = frame.to_le_stream().collect();

        #[expect(unsafe_code)]
        // SAFETY: We trust that the caller upholds the safety invariants mentioned above.
        unsafe {
            apis_saltans_hw::Frame::new(metadata, payload)
        }
    }

    async fn handle_match_desc_req(&self, src_address: u16, seq: u8, match_desc_req: MatchDescReq) {
        let payload = MatchDescRsp::new(
            Status::Success,
            0x0000,
            self.endpoints
                .iter()
                .filter_map(|endpoint| {
                    if match_desc_req.matches(endpoint) {
                        Some(u8::from(endpoint.endpoint()))
                    } else {
                        None
                    }
                })
                .collect(),
        );

        if let Err(error) = self
            .unicast(seq, src_address, Payload::zdp(payload).into_command())
            .await
        {
            error!("Failed to send Match_Desc_rsp: {error:?}");
        }
    }

    async fn handle_device_annce(&self, device_annce: DeviceAnnce) {
        let Some(discovery) = self.discovery.upgrade() else {
            trace!("Discovery channel dropped");
            return;
        };

        discovery
            .send(discovery::Message::DeviceAnnounced {
                address: Address::new(device_annce.ieee_addr(), device_annce.nwk_addr()),
                capabilities: device_annce.capabilities(),
            })
            .await
            .unwrap_or_else(|error| {
                error!("Failed to send device announcement: {error:?}");
            });
    }
}

impl<T> Transceiver<T>
where
    T: Ncp + Send + Sync + 'static,
{
    /// Start the ZDP transceiver.
    pub fn spawn(
        ncp: T,
        discovery: WeakSender<discovery::Message>,
        endpoints: &[SimpleDescriptor],
    ) -> tokio::sync::mpsc::Sender<Message> {
        let (zdp_tx, zdp_rx) = tokio::sync::mpsc::channel(MPSC_CHANNEL_SIZE);
        spawn(Self::new(ncp, discovery, endpoints.into()).run(zdp_rx));
        zdp_tx
    }
}
