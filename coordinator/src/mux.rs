use bytes::Bytes;
use log::{error, trace, warn};
use tokio::spawn;
use tokio::sync::mpsc::{Receiver, Sender};
use zb_aps::data::Frame;
use zb_aps::{Assembler, Data};
use zb_hw::{Event as HardwareEvent, Ncp, NcpHandle};
use zb_nwk::{Envelope, Source};

use self::aps_payload::ApsPayload;
use crate::{Device, Event as ApplicationEvent, Network, NetworkError, zcl, zdp};

mod aps_payload;

/// Event multiplexer.
#[derive(Debug)]
pub struct Mux {
    ncp: NcpHandle,
    events: Sender<ApplicationEvent>,
    zcl: Sender<zcl::Message>,
    zdp: Sender<zdp::Message>,
    transactions: Assembler,
}

impl Mux {
    /// Create a new multiplexer.
    pub fn new(
        ncp: NcpHandle,
        events: Sender<ApplicationEvent>,
        zcl: Sender<zcl::Message>,
        zdp: Sender<zdp::Message>,
    ) -> Self {
        Self {
            ncp,
            events,
            zcl,
            zdp,
            transactions: Assembler::default(),
        }
    }

    /// Start the multiplexer.
    pub fn spawn(
        ncp: NcpHandle,
        hw_events: Receiver<HardwareEvent>,
        events_out: Sender<ApplicationEvent>,
        zcl_tx: Sender<zcl::Message>,
        zdp_tx: Sender<zdp::Message>,
    ) {
        spawn(Self::new(ncp, events_out, zcl_tx, zdp_tx).run(hw_events));
    }

    /// Run the multiplexer.
    pub async fn run(mut self, mut messages: Receiver<HardwareEvent>) {
        while let Some(event) = messages.recv().await {
            self.multiplex(event).await;
        }
    }

    async fn multiplex(&mut self, event: HardwareEvent) {
        match event {
            HardwareEvent::NetworkUp => {
                trace!("Network is up");
                self.events
                    .send(ApplicationEvent::Network(Network::Up))
                    .await
                    .unwrap_or_else(drop);
            }
            HardwareEvent::NetworkDown => {
                trace!("Network is down");
                self.events
                    .send(ApplicationEvent::Network(Network::Down))
                    .await
                    .unwrap_or_else(drop);
            }
            HardwareEvent::NetworkOpened => {
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
            HardwareEvent::NetworkClosed => {
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
            HardwareEvent::DeviceJoined(address) => {
                trace!("Device joined: {address}");
                self.events
                    .send(ApplicationEvent::Device(Device::Joined(address)))
                    .await
                    .unwrap_or_else(drop);
            }
            HardwareEvent::DeviceRejoined { address, secured } => {
                trace!("Device joined: {address} (secured: {secured})");
                self.events
                    .send(ApplicationEvent::Device(Device::Rejoined {
                        address,
                        secured,
                    }))
                    .await
                    .unwrap_or_else(drop);
            }
            HardwareEvent::DeviceLeft(address) => {
                trace!("Device left: {address}");
                self.events
                    .send(ApplicationEvent::Device(Device::Left(address)))
                    .await
                    .unwrap_or_else(drop);
            }
            HardwareEvent::MessageReceived(envelope) => {
                trace!("Message received: {envelope:?}");
                self.handle_nwk_envelope(envelope).await;
            }
            HardwareEvent::RouteError(error) => {
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

    async fn handle_nwk_envelope(&mut self, envelope: Envelope<Data<Bytes>>) {
        let source = envelope.source();
        let header = envelope.payload().header();
        let metadata = envelope.metadata();

        if let Some(frame) = self.transactions.add(envelope) {
            match frame.parse() {
                Ok(frame) => self.forward_received_message(source, frame).await,
                Err(error) => warn!("Failed to parse APS frame: {error}"),
            }
        } else {
            self.ncp
                .send_reply(source.node_id(), header, metadata)
                .await
                .unwrap_or_else(|error| {
                    error!("Failed to send response to fragmented frame: {error}");
                })
        }
    }

    async fn forward_received_message(&self, source: Source, aps_frame: Frame<ApsPayload>) {
        let (header, payload) = aps_frame.into_parts();

        match payload {
            ApsPayload::Zcl(frame) => {
                #[expect(unsafe_code)]
                // SAFETY: We reconstruct the frame from its original parts.
                let frame = unsafe { Frame::new_unchecked(header, frame) };

                self.zcl
                    .send(zcl::Message::Received { source, frame })
                    .await
                    .unwrap_or_else(|error| {
                        trace!("Failed to send ZCL message: {error}");
                    });
            }
            ApsPayload::Zdp(frame) => {
                #[expect(unsafe_code)]
                // SAFETY: We reconstruct the frame from its original parts.
                let frame = unsafe { Frame::new_unchecked(header, frame) };

                self.zdp
                    .send(zdp::Message::Received { source, frame })
                    .await
                    .unwrap_or_else(|error| {
                        trace!("Failed to send ZDP message: {error}");
                    });
            }
        }
    }
}
