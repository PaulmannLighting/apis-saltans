use tokio::sync::oneshot::Sender;
use zcl::Cluster;
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
        /// An optional manufacturer code.
        manufacturer_code: Option<u16>,
        /// ZCL payload.
        payload: Box<Cluster>,
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
