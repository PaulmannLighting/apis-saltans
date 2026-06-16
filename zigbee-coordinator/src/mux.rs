use aps::data::Frame;
use log::trace;
use tokio::sync::mpsc::error::SendError;
use tokio::sync::mpsc::{Receiver, Sender};
use zigbee_hw::{Command, Event};

pub use self::message::Message;
use crate::discovery;
use crate::transceiver::{zcl, zdp};

mod message;

/// Event multiplexer.
#[derive(Debug)]
pub struct Mux {
    discovery: Sender<discovery::Message>,
    zcl: Sender<zcl::Message>,
    zdp: Sender<zdp::Message>,
    subscribers: Vec<Sender<Event>>,
}

impl Mux {
    /// Create a new multiplexer.
    pub const fn new(
        discovery: Sender<discovery::Message>,
        zcl: Sender<zcl::Message>,
        zdp: Sender<zdp::Message>,
    ) -> Self {
        Self {
            discovery,
            zcl,
            zdp,
            subscribers: Vec::new(),
        }
    }

    /// Run the multiplexer.
    pub async fn run(mut self, mut messages: Receiver<Message>) {
        while let Some(message) = messages.recv().await {
            match message {
                Message::Event(event) => self.multiplex(event).await,
                Message::Subscribe { sender } => self.subscribers.push(sender),
            }
        }
    }

    async fn multiplex(&mut self, event: Event) {
        self.forward_to_subscribers(&event).await;

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
            Event::MessageReceived {
                src_address,
                aps_frame,
            } => {
                self.forward_received_message(src_address, *aps_frame).await;
            }
            other => trace!("Received unknown event: {other:?}"),
        }
    }

    async fn forward_to_subscribers(&mut self, event: &Event) {
        for subscriber in &self.subscribers {
            if let Err(error) = subscriber.send(event.clone()).await {
                trace!("Subscriber went away: {error}");
            }
        }

        self.subscribers
            .retain(|subscriber| !subscriber.is_closed());
    }

    async fn forward_received_message(&self, src_address: u16, aps_frame: Frame<Command>) {
        let (_, payload) = aps_frame.into_parts();

        match payload {
            Command::Zcl(frame) => {
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

/// A handle on the multiplexer actor.
pub trait Handle {
    /// Subscribe to an event.
    ///
    /// # Errors
    ///
    /// Returns a [`SendError`] if the multiplexer is no longer running.
    fn subscribe(
        &self,
        sender: Sender<Event>,
    ) -> impl Future<Output = Result<(), SendError<Message>>>;
}

impl Handle for Sender<Message> {
    async fn subscribe(&self, sender: Sender<Event>) -> Result<(), SendError<Message>> {
        self.send(Message::Subscribe { sender }).await
    }
}
