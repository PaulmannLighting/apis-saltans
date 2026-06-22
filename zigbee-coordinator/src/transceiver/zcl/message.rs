use aps::Data;
use tokio::sync::oneshot::{Receiver, Sender};
use zcl::{Cluster, Frame};
use zigbee::Endpoint;
use zigbee_hw::Error;

pub use self::payload::Payload;

mod payload;

/// Messages exchanged with the transceiver actor.
#[derive(Debug)]
pub enum Message {
    /// A hardware-level event.
    Received {
        /// The PAN ID of the sender.
        src_address: u16,
        /// The APS frame.
        frame: Box<Data<Frame<Cluster>>>,
    },
    /// Unicast a message.
    Unicast {
        /// The destination address.
        short_id: u16,
        /// The destination endpoint.
        endpoint: Endpoint,
        /// The payload.
        payload: Box<Payload<Cluster>>,
        /// The response channel.
        response: Sender<Result<(), Error>>,
    },
    /// Unicast a message.
    Multicast {
        /// The destination group ID.
        group_id: u16,
        /// The number of hops.
        hops: u8,
        /// The multicast radius.
        radius: u8,
        /// The payload.
        payload: Box<Payload<Cluster>>,
        /// The response channel.
        response: Sender<Result<(), Error>>,
    },
    /// Communicate a unicast with an expected response.
    Communicate {
        /// The destination address.
        short_id: u16,
        /// The destination endpoint.
        endpoint: Endpoint,
        /// The payload.
        payload: Box<Payload<Cluster>>,
        /// The response channel.
        response: Sender<Result<Receiver<Cluster>, Error>>,
    },
}
