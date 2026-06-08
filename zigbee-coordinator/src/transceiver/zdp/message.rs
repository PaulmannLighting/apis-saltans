use tokio::sync::oneshot::Sender;
use zdp::Command;
use zigbee::{Address, Endpoint};
use zigbee_hw::{Error, Event, Metadata};

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
        /// APS metadata for transmission.
        metadata: Metadata,
        /// ZDP command.
        command: Box<Command>,
        /// The response channel.
        response: Sender<Result<(), Error>>,
    },
    /// Subscribe to the response multiplexer.
    Subscribe {
        /// ZDP sequence number.
        seq: u8,
        /// ZDP response channel.
        response: Sender<Command>,
    },
}

impl From<Event> for Message {
    fn from(event: Event) -> Self {
        Self::Event(event)
    }
}
