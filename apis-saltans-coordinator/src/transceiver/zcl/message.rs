use apis_saltans_aps::Data;
use apis_saltans_core::Application;
use apis_saltans_hw::Error;
use apis_saltans_nwk::Source;
use apis_saltans_zcl::{Cluster, Frame};
use tokio::sync::oneshot::{Receiver, Sender};

pub use self::payload::Payload;

mod payload;

/// Messages exchanged with the transceiver actor.
#[derive(Debug)]
pub enum Message {
    /// A hardware-level event.
    Received {
        /// The NWK source information of the frame.
        source: Source,
        /// The APS frame.
        frame: Data<Frame<Cluster>>,
    },

    /// Unicast a message.
    Unicast {
        /// The destination address.
        short_id: u16,
        /// The destination endpoint.
        endpoint: Application,
        /// The payload.
        payload: Payload<Cluster>,
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
        payload: Payload<Cluster>,
        /// The response channel.
        response: Sender<Result<(), Error>>,
    },

    /// Communicate a unicast with an expected response.
    Communicate {
        /// The destination address.
        short_id: u16,
        /// The destination endpoint.
        endpoint: Application,
        /// The payload.
        payload: Payload<Cluster>,
        /// The response channel.
        response: Sender<Result<Receiver<Cluster>, Error>>,
    },
}
