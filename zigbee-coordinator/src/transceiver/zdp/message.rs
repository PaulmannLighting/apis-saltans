use tokio::sync::oneshot::{Receiver, Sender};
use zdp::Command;
use zigbee::Address;
use zigbee_hw::{Error, Event};

pub use self::payload::Payload;

mod payload;

/// Messages exchanged with the transceiver actor.
#[derive(Debug)]
pub enum Message {
    /// A hardware-level event.
    Event(Event),
    /// Unicast a message.
    Unicast {
        /// The destination address.
        short_id: u16,
        /// The payload.
        payload: Box<Payload<Command>>,
        /// The response channel.
        response: Sender<Result<(), Error>>,
    },
    /// Communicate a unicast with an expected response.
    Communicate {
        /// The destination address.
        short_id: u16,
        /// The payload.
        payload: Box<Payload<Command>>,
        /// The response channel.
        response: Sender<Result<Receiver<Command>, Error>>,
    },
}

impl From<Event> for Message {
    fn from(event: Event) -> Self {
        Self::Event(event)
    }
}
