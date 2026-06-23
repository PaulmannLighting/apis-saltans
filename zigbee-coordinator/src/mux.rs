use aps::Transactions;
use aps::data::Frame;
use log::trace;
use tokio::sync::mpsc::{Receiver, Sender};
use zigbee_hw::Event;

use crate::aps_payload::ApsPayload;
use crate::transceiver::{zcl, zdp};
use crate::{discovery, network_manager};

/// Event multiplexer.
#[derive(Debug)]
pub struct Mux {
    zcl: Sender<zcl::Message>,
    zdp: Sender<zdp::Message>,
    discovery: Sender<discovery::Message>,
    network_manager: Sender<network_manager::Message>,
    transactions: Transactions,
}

impl Mux {
    /// Create a new multiplexer.
    pub const fn new(
        zcl: Sender<zcl::Message>,
        zdp: Sender<zdp::Message>,
        discovery: Sender<discovery::Message>,
        network_manager: Sender<network_manager::Message>,
    ) -> Self {
        Self {
            zcl,
            zdp,
            discovery,
            network_manager,
            transactions: Transactions::new(),
        }
    }

    /// Run the multiplexer.
    pub async fn run(mut self, mut messages: Receiver<Event>) {
        while let Some(event) = messages.recv().await {
            self.multiplex(event).await;
        }
    }

    async fn multiplex(&mut self, event: Event) {
        match event {
            Event::DeviceJoined(address) => {
                self.discovery
                    .send(discovery::Message::DeviceJoined(address))
                    .await
                    .unwrap_or_else(|error| {
                        trace!("Failed to send device joined message: {error}");
                    });
            }
            Event::DeviceRejoined { address, secured } => {
                self.discovery
                    .send(discovery::Message::DeviceRejoined { address, secured })
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
            Event::MessageReceived {
                src_address,
                aps_frame,
            } => {
                self.handle_aps_frame(src_address, aps_frame).await;
            }
            other => trace!("Received unknown event: {other:?}"),
        }
    }

    async fn handle_aps_frame(&mut self, src_address: u16, aps_frame: Frame<Vec<u8>>) {
        if let Some(frame) = self.transactions.add(aps_frame) {
            match frame.parse() {
                Ok(frame) => self.forward_received_message(src_address, frame).await,
                Err(error) => trace!("Failed to parse APS frame: {error}"),
            }
        }
    }

    async fn forward_received_message(&self, src_address: u16, aps_frame: Frame<ApsPayload>) {
        let (header, payload) = aps_frame.into_parts();

        match payload {
            ApsPayload::Zcl(frame) => {
                #[expect(unsafe_code)]
                // SAFETY: We reconstructed the frame from its original parts above.
                let frame = unsafe { Frame::new_unchecked(header, frame) };

                self.zcl
                    .send(zcl::Message::Received {
                        src_address,
                        frame: frame.into(),
                    })
                    .await
                    .unwrap_or_else(|error| {
                        trace!("Failed to send ZCL message: {error}");
                    });
            }
            ApsPayload::Zdp(frame) => {
                self.zdp
                    .send(zdp::Message::Received {
                        src_address,
                        frame: frame.into(),
                    })
                    .await
                    .unwrap_or_else(|error| {
                        trace!("Failed to send ZDP message: {error}");
                    });
            }
        }
    }
}
