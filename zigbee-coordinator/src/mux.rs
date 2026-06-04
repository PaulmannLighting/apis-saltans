use std::collections::BTreeMap;

use aps::data::Header;
use log::error;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::oneshot;
use zcl::Cluster;
use zigbee_hw::{Command, Event};

use crate::log_send_error::log_send_error;
use crate::zcl_message::ZclMessage;
use crate::{binding, discovery, network_manager, transmitter};

/// Event multiplexer
#[derive(Debug)]
pub struct Mux {
    network_manager: Sender<network_manager::Message>,
    discovery: Sender<discovery::Message>,
    binding: Sender<binding::Message>,
    transmitter: Sender<transmitter::Message>,
    zcl_responses: BTreeMap<u8, oneshot::Sender<ZclMessage>>,
    subscribers: Vec<Sender<Event>>,
}

impl Mux {
    /// Create a new event multiplexer.
    #[must_use]
    pub const fn new(
        network_manager: Sender<network_manager::Message>,
        discovery: Sender<discovery::Message>,
        binding: Sender<binding::Message>,
        transmitter: Sender<transmitter::Message>,
    ) -> Self {
        Self {
            network_manager,
            discovery,
            binding,
            transmitter,
            zcl_responses: BTreeMap::new(),
            subscribers: Vec::new(),
        }
    }

    pub async fn run(mut self, mut incoming: Receiver<Event>) {
        while let Some(event) = incoming.recv().await {
            self.multiplex_event(&event).await;

            match event {
                Event::NetworkUp => self
                    .network_manager
                    .send(network_manager::Message::NetworkUp)
                    .await
                    .unwrap_or_else(log_send_error("network manager")),
                Event::NetworkDown => self
                    .network_manager
                    .send(network_manager::Message::NetworkDown)
                    .await
                    .unwrap_or_else(log_send_error("network manager")),
                Event::NetworkOpened => self
                    .network_manager
                    .send(network_manager::Message::NetworkOpened)
                    .await
                    .unwrap_or_else(log_send_error("network manager")),
                Event::NetworkClosed => self
                    .network_manager
                    .send(network_manager::Message::NetworkClosed)
                    .await
                    .unwrap_or_else(log_send_error("network manager")),
                Event::DeviceJoined {
                    ieee_address,
                    short_id,
                } => {
                    self.discovery
                        .send(discovery::Message::DeviceJoined {
                            ieee_address,
                            short_id,
                        })
                        .await
                        .unwrap_or_else(log_send_error("discovery"));
                    self.binding
                        .send(binding::Message::DeviceJoined {
                            ieee_address,
                            short_id,
                        })
                        .await
                        .unwrap_or_else(log_send_error("binding"));
                }
                Event::DeviceRejoined {
                    ieee_address,
                    short_id,
                    secured,
                } => {
                    self.discovery
                        .send(discovery::Message::DeviceRejoined {
                            ieee_address,
                            short_id,
                            secured,
                        })
                        .await
                        .unwrap_or_else(log_send_error("discovery"));
                    self.binding
                        .send(binding::Message::DeviceRejoined {
                            ieee_address,
                            short_id,
                            secured,
                        })
                        .await
                        .unwrap_or_else(log_send_error("binding"));
                }
                Event::DeviceLeft {
                    ieee_address,
                    short_id,
                } => {
                    self.discovery
                        .send(discovery::Message::DeviceLeft {
                            ieee_address,
                            short_id,
                        })
                        .await
                        .unwrap_or_else(log_send_error("discovery"));
                    self.binding
                        .send(binding::Message::DeviceLeft {
                            ieee_address,
                            short_id,
                        })
                        .await
                        .unwrap_or_else(log_send_error("binding"));
                }
                Event::MessageReceived {
                    src_address,
                    aps_frame,
                } => {
                    let (aps_header, payload) = aps_frame.into_parts();

                    match payload {
                        Command::Zdp(command) => todo!("Handle ZDP command: {command:?}"),
                        Command::Zcl(frame) => {
                            self.send_zcl_response(src_address, aps_header, frame);
                        }
                    }
                }
            }
        }
    }

    /// Multiplex an event to all subscribers, removing any closed channels.
    async fn multiplex_event(&mut self, event: &Event) {
        self.subscribers.retain(|sender| !sender.is_closed());

        for subscriber in &self.subscribers {
            subscriber
                .send(event.clone())
                .await
                .unwrap_or_else(log_send_error("subscriber"));
        }
    }

    /// Send a ZCL response frame if a receiver is waiting for it.
    fn send_zcl_response(
        &mut self,
        src_address: u16,
        aps_header: Header,
        frame: zcl::Frame<Cluster>,
    ) {
        let seq = frame.header().seq();

        if let Some(sender) = self.zcl_responses.remove(&seq) {
            sender
                .send(ZclMessage::new(src_address, aps_header, frame))
                .unwrap_or_else(|message| {
                    error!("Failed to send ZCL response for sequence number {seq}: {message:?}");
                });
        }
    }
}
