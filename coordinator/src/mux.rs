use bytes::Bytes;
use log::{trace, warn};
use tokio::spawn;
use tokio::sync::mpsc::{Receiver, Sender};
use zb_aps::data::Frame;
use zb_aps::{Assembler, Data};
use zb_core::destination;
use zb_hw::{
    ApsEvent as HardwareApsEvent, DeviceEvent as HardwareDeviceEvent, Event as HardwareEvent,
    NetworkEvent as HardwareNetworkEvent,
};
use zb_nwk::{Envelope, Source};

use self::aps_payload::ApsPayload;
use crate::{Device, Event as ApplicationEvent, Event, Network, NetworkError, aps, zcl, zdp};

mod aps_payload;

/// Event multiplexer.
#[derive(Debug)]
pub struct Mux {
    events: Sender<ApplicationEvent>,
    aps: aps::Aps,
    zcl: Sender<zcl::Message>,
    zdp: Sender<zdp::Message>,
    transactions: Assembler,
}

impl Mux {
    /// Create a new multiplexer.
    pub fn new(
        events: Sender<ApplicationEvent>,
        aps: aps::Aps,
        zcl: Sender<zcl::Message>,
        zdp: Sender<zdp::Message>,
    ) -> Self {
        Self {
            events,
            aps,
            zcl,
            zdp,
            transactions: Assembler::default(),
        }
    }

    /// Start the multiplexer.
    pub fn spawn(
        hw_events: Receiver<HardwareEvent>,
        events_out: Sender<ApplicationEvent>,
        aps: aps::Aps,
        zcl_tx: Sender<zcl::Message>,
        zdp_tx: Sender<zdp::Message>,
    ) {
        spawn(Self::new(events_out, aps, zcl_tx, zdp_tx).run(hw_events));
    }

    /// Run the multiplexer.
    pub async fn run(mut self, mut messages: Receiver<HardwareEvent>) {
        while let Some(event) = messages.recv().await {
            self.multiplex(event).await;
        }
    }

    async fn multiplex(&mut self, event: HardwareEvent) {
        match event {
            HardwareEvent::Network(event) => self.multiplex_network_event(event).await,
            HardwareEvent::Device(event) => self.multiplex_device_event(event).await,
            HardwareEvent::Aps(event) => self.multiplex_aps_event(event).await,
        }
    }

    async fn multiplex_network_event(&self, event: HardwareNetworkEvent) {
        match event {
            HardwareNetworkEvent::Up => {
                trace!("Network is up");
                self.events
                    .send(ApplicationEvent::Network(Network::Up))
                    .await
                    .unwrap_or_else(drop);
            }
            HardwareNetworkEvent::Down => {
                trace!("Network is down");
                self.events
                    .send(ApplicationEvent::Network(Network::Down))
                    .await
                    .unwrap_or_else(drop);
            }
            HardwareNetworkEvent::Opened => {
                trace!("Network has been opened");
                self.zdp
                    .send(zdp::Message::NetworkOpened)
                    .await
                    .unwrap_or_else(|error| {
                        trace!("Failed to send ZDP message: {error}");
                    });
                self.events
                    .send(ApplicationEvent::Network(Network::Opened))
                    .await
                    .unwrap_or_else(drop);
            }
            HardwareNetworkEvent::Closed => {
                trace!("Network has been closed");
                self.zdp
                    .send(zdp::Message::NetworkClosed)
                    .await
                    .unwrap_or_else(|error| {
                        trace!("Failed to send ZDP message: {error}");
                    });
                self.events
                    .send(ApplicationEvent::Network(Network::Closed))
                    .await
                    .unwrap_or_else(drop);
            }
            HardwareNetworkEvent::RouteError(error) => {
                trace!("Route error: {error}");
                self.events
                    .send(ApplicationEvent::Network(Network::Error(
                        NetworkError::Route(error),
                    )))
                    .await
                    .unwrap_or_else(drop);
            }
        }
    }

    async fn multiplex_device_event(&self, event: HardwareDeviceEvent) {
        match event {
            HardwareDeviceEvent::Joined(address) => {
                trace!("Device joined: {address}");
                self.events
                    .send(ApplicationEvent::Device(Device::Joined(address)))
                    .await
                    .unwrap_or_else(drop);
            }
            HardwareDeviceEvent::Rejoined { address, secured } => {
                trace!("Device joined: {address} (secured: {secured})");
                self.events
                    .send(ApplicationEvent::Device(Device::Rejoined {
                        address,
                        secured,
                    }))
                    .await
                    .unwrap_or_else(drop);
            }
            HardwareDeviceEvent::Left(address) => {
                trace!("Device left: {address}");
                self.events
                    .send(ApplicationEvent::Device(Device::Left(address)))
                    .await
                    .unwrap_or_else(drop);
            }
        }
    }

    async fn multiplex_aps_event(&mut self, event: HardwareApsEvent) {
        match event {
            HardwareApsEvent::MessageReceived(envelope) => {
                trace!("Message received: {envelope:?}");
                self.handle_nwk_envelope(envelope).await;
            }
            HardwareApsEvent::Ack(sequence) => {
                trace!("APS acknowledgement received for sequence: {sequence}");
                self.aps.ack(sequence).await.unwrap_or_else(|error| {
                    trace!("Failed to send APS acknowledgement: {error}");
                });
            }
            HardwareApsEvent::Nak { sequence, error } => {
                trace!("APS negative acknowledgement received for sequence {sequence}: {error}");
                self.aps.nak(sequence, error).await.unwrap_or_else(|error| {
                    trace!("Failed to send APS negative acknowledgement: {error}");
                });
            }
        }
    }

    async fn handle_nwk_envelope(&mut self, envelope: Envelope<Data<Bytes>>) {
        let source = envelope.source();

        if let Some(frame) = self.transactions.add(envelope) {
            match frame.parse() {
                Ok(frame) => self.forward_received_message(source, frame).await,
                Err(error) => warn!("Failed to parse APS frame: {error}"),
            }
        }
    }

    async fn forward_received_message(&self, source: Source, aps_frame: Frame<ApsPayload>) {
        let (header, payload) = aps_frame.into_parts();

        match payload {
            ApsPayload::Zcl(frame) => {
                let frame = Frame::new(header, frame);

                self.zcl
                    .send(zcl::Message::Received { source, frame })
                    .await
                    .unwrap_or_else(|error| {
                        trace!("Failed to send ZCL message: {error}");
                    });
            }
            ApsPayload::Zdp(frame) => {
                let frame = Frame::new(header, frame);

                self.zdp
                    .send(zdp::Message::Received { source, frame })
                    .await
                    .unwrap_or_else(|error| {
                        trace!("Failed to send ZDP message: {error}");
                    });
            }
            ApsPayload::KeepAlive => {
                let Ok(device_id) = source.node_id().try_into().inspect_err(|id| {
                    warn!("Keep-Alive packet from invalid device id: {id:#06X}");
                }) else {
                    return;
                };

                self.events
                    .send(Event::Device(Device::KeepAlive(destination::Device::new(
                        device_id,
                        header.source_endpoint(),
                    ))))
                    .await
                    .unwrap_or_else(drop);
            }
        }
    }
}
