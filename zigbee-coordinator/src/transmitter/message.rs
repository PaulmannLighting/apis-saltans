use std::time::Duration;

use tokio::sync::oneshot::Sender;
use zcl::Cluster;
use zigbee::Endpoint;
use zigbee_hw::{Error, Event, Metadata};

pub use self::payload::Payload;

mod payload;

/// Messages exchanged with the transmitter actor.
#[derive(Debug)]
pub enum Message {
    /// A hardware-level event.
    Event(Event),
    /// Allow new devices to join the network.
    AllowJoins {
        /// The duration for which to allow joins.
        duration: Duration,
    },
    Unicast {
        /// The destination address.
        short_id: u16,
        /// The destination endpoint.
        endpoint: Endpoint,
        /// APS metadata for transmission.
        metadata: Metadata,
        /// The payload.
        payload: Box<Payload>,
        /// The response channel.
        response: Sender<Result<(), Error>>,
    },
    /// Subscribe to the response multiplexer.
    Subscribe {
        /// ZCL sequence number.
        seq: u8,
        /// ZCL response channel.
        response: Sender<Cluster>,
    },
}

impl From<Event> for Message {
    fn from(event: Event) -> Self {
        Self::Event(event)
    }
}
