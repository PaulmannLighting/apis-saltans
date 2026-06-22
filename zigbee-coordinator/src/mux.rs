use aps::data::Frame;
use log::trace;
use tokio::sync::mpsc::{Receiver, Sender};
use zigbee_hw::{Command, Event};

use crate::transceiver::{zcl, zdp};
use crate::{discovery, network_manager};

/// Event multiplexer.
#[derive(Debug)]
pub struct Mux {
    zcl: Sender<zcl::Message>,
    zdp: Sender<zdp::Message>,
    discovery: Sender<discovery::Message>,
    network_manager: Sender<network_manager::Message>,
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
        }
    }

    /// Run the multiplexer.
    pub async fn run(self, mut messages: Receiver<Event>) {
        while let Some(event) = messages.recv().await {
            self.multiplex(event).await;
        }
    }

    async fn multiplex(&self, event: Event) {
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
                self.forward_received_message(src_address, *aps_frame).await;
            }
            other => trace!("Received unknown event: {other:?}"),
        }
    }

    async fn forward_received_message(&self, src_address: u16, aps_frame: Frame<Command>) {
        let (header, payload) = aps_frame.into_parts();

        match payload {
            Command::Zcl(frame) => {
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
            Command::Zdp(frame) => {
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
