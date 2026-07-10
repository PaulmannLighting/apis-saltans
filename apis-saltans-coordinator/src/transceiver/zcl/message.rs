use apis_saltans_aps::Data;
use apis_saltans_core::Destination;
use apis_saltans_core::destination::Device;
use apis_saltans_hw::Error;
use apis_saltans_nwk::Source;
use apis_saltans_zcl::{Cluster, Frame};
use tokio::sync::oneshot::{Receiver, Sender};

pub use super::{Metadata, Payload};

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
