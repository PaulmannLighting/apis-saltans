use tokio::sync::oneshot::{Receiver, Sender};
use zcl::Cluster;
use zigbee::{Address, Endpoint};
use zigbee_hw::{Error, Event};

pub use crate::transceiver::aps::Frame;

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
        /// The payload
        payload: Box<Frame<Cluster>>,
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
        payload: Box<Frame<Cluster>>,
        /// The response channel.
        response: Sender<Result<Receiver<Cluster>, Error>>,
    },
}

impl From<Event> for Message {
    fn from(event: Event) -> Self {
        Self::Event(event)
    }
}
