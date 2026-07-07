use apis_saltans_aps::data::Frame;
use apis_saltans_aps::{Assembler, Data};
use apis_saltans_hw::Event;
use apis_saltans_nwk::{Envelope, Source};
use bytes::Bytes;
use log::{error, trace, warn};
use tokio::spawn;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::aps_payload::ApsPayload;
use crate::network_manager;
use crate::transceiver::{zcl, zdp};

/// Event multiplexer.
#[derive(Debug)]
pub struct Mux {
    zcl: Sender<zcl::Message>,
    zdp: Sender<zdp::Message>,
    network_manager: Sender<network_manager::Message>,
    transactions: Assembler,
}

impl Mux {
    /// Create a new multiplexer.
    pub fn new(
        zcl: Sender<zcl::Message>,
        zdp: Sender<zdp::Message>,
        network_manager: Sender<network_manager::Message>,
    ) -> Self {
        Self {
            zcl,
            zdp,
            network_manager,
            transactions: Assembler::default(),
        }
    }

    /// Start the multiplexer.
    pub fn spawn(
        events: Receiver<Event>,
        zcl_tx: Sender<zcl::Message>,
        zdp_tx: Sender<zdp::Message>,
        network_manager: Sender<network_manager::Message>,
    ) {
        spawn(Self::new(zcl_tx, zdp_tx, network_manager).run(events));
    }

    /// Run the multiplexer.
    pub async fn run(mut self, mut messages: Receiver<Event>) {
        while let Some(event) = messages.recv().await {
            self.multiplex(event).await;
        }
    }

    async fn multiplex(&mut self, event: Event) {
        match event {
            Event::NetworkUp => {
                trace!("Network is up");
            }
            Event::NetworkDown => {
                trace!("Network is down");
            }
            Event::NetworkOpened => {
                trace!("Network has been opened");
                self.network_manager
                    .send(network_manager::Message::NetworkOpened)
                    .await
                    .unwrap_or_else(|error| {
                        trace!("Failed to send network opened message: {error}");
                    });
            }
            Event::NetworkClosed => {
                trace!("Network has been closed");
                self.network_manager
                    .send(network_manager::Message::NetworkClosed)
                    .await
                    .unwrap_or_else(|error| {
                        trace!("Failed to send network closed message: {error}");
                    });
            }
            Event::DeviceJoined(address) => {
                self.network_manager
                    .send(network_manager::Message::DeviceJoined {
                        address,
                        secured: None,
                    })
                    .await
                    .unwrap_or_else(|error| {
                        trace!("Failed to send device joined message: {error}");
                    });
            }
            Event::DeviceRejoined { address, secured } => {
                self.network_manager
                    .send(network_manager::Message::DeviceJoined {
                        address,
                        secured: Some(secured),
                    })
                    .await
                    .unwrap_or_else(|error| {
                        trace!("Failed to send device rejoined message: {error}");
                    });
            }
            Event::DeviceLeft(address) => self
                .network_manager
                .send(network_manager::Message::RemoveDevice(address))
                .await
                .unwrap_or_else(|error| {
                    trace!("Failed to send device left message: {error}");
                }),
            Event::MessageReceived(envelope) => {
                self.handle_nwk_envelope(envelope).await;
            }
            Event::RouteError(error) => {
                trace!("{error}");
                self.network_manager
                    .send(network_manager::Message::RouteError(error))
                    .await
                    .unwrap_or_else(|error| {
                        error!("Failed to send route error message: {error}");
                    });
            }
        }
    }

    async fn handle_nwk_envelope(&mut self, envelope: Envelope<Data<Bytes>>) {
        trace!("Received NWK envelope: {envelope:?}");
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
                #[expect(unsafe_code)]
                // SAFETY: We reconstructed the frame from its original parts above.
                let frame = unsafe { Frame::new_unchecked(header, frame) };

                self.zcl
                    .send(zcl::Message::Received { source, frame })
                    .await
                    .unwrap_or_else(|error| {
                        trace!("Failed to send ZCL message: {error}");
                    });
            }
            ApsPayload::Zdp(frame) => {
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
