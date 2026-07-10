use tokio::sync::oneshot::{Receiver, Sender};
use zb_aps::Data;
use zb_core::Destination;
use zb_core::destination::Device;
use zb_hw::Error;
use zb_nwk::Source;
use zb_zcl::{Cluster, Frame};

pub use super::Payload;

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
    Transmit {
        destination: Destination,
        payload: Payload,
        /// The response channel.
        response: Sender<Result<(), Error>>,
    },

    /// Communicate a unicast with an expected response.
    Communicate {
        destination: Device,
        payload: Payload,
        /// The response channel.
        response: Sender<Result<Receiver<Cluster>, Error>>,
    },
}
