use tokio::sync::oneshot::{Receiver, Sender};
use zdp::Command;
use zigbee::{Address, Endpoint};
use zigbee_hw::{Error, Event};

use crate::transceiver::aps::Frame;

/// Messages exchanged with the transceiver actor.
#[derive(Debug)]
pub enum Message {
    /// A hardware-level event.
    Event(Event),
    /// Unicast a message.
    Unicast {
        /// The destination address.
        address: Address,
        /// The destination endpoint.
        endpoint: Endpoint,
        /// ZDP command.
        frame: Box<Frame<Command>>,
        /// The response channel.
        response: Sender<Result<(), Error>>,
    },
    /// Communicate a unicast with an expected response.
    Communicate {
        /// The destination address.
        address: Address,
        /// The destination endpoint.
        endpoint: Endpoint,
        /// The payload
        frame: Box<Frame<Command>>,
        /// The response channel.
        response: Sender<Result<Receiver<Command>, Error>>,
    },
}

impl From<Event> for Message {
    fn from(event: Event) -> Self {
        Self::Event(event)
    }
}
