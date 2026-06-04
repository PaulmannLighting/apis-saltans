use std::collections::BTreeMap;

use log::error;
use tokio::sync::mpsc::Receiver;
use tokio::sync::oneshot::Sender;
use zcl::Cluster;
use zigbee_hw::{Command, Event};

pub use self::message::Message;
use self::subscribers::Subscribers;

mod message;
mod subscribers;

/// Event multiplexer
#[derive(Debug, Default)]
pub struct Mux {
    zcl_responses: BTreeMap<u8, Sender<Cluster>>,
    subscribers: Subscribers,
}

impl Mux {
    pub async fn run(mut self, mut messages: Receiver<Message>) {
        while let Some(message) = messages.recv().await {
            match message {
                Message::SubscribeZclResponse { seq, response } => {
                    self.zcl_responses.insert(seq, response);
                }
                Message::SubscribeEvent { sender } => self.subscribers.add(sender),
                Message::Event(event) => self.handle_event(event).await,
            }
        }
    }

    async fn handle_event(&mut self, event: Event) {
        self.subscribers.send(&event).await;

        if let Event::MessageReceived { aps_frame, .. } = event {
            let (_header, payload) = aps_frame.into_parts();

            match payload {
                Command::Zdp(command) => todo!("Handle ZDP command: {command:?}"),
                Command::Zcl(frame) => {
                    self.send_zcl_response(frame);
                }
            }
        }
    }

    /// Send a ZCL response frame if a receiver is waiting for it.
    fn send_zcl_response(&mut self, frame: zcl::Frame<Cluster>) {
        let seq = frame.header().seq();

        if let Some(sender) = self.zcl_responses.remove(&seq) {
            sender.send(frame.into_payload()).unwrap_or_else(|message| {
                error!("Failed to send ZCL response for sequence number {seq}: {message:?}");
            });
        }
    }
}
