use tokio::sync::oneshot::{Receiver, Sender};
use apis_saltans_zdp::{Command, Frame};
use apis_saltans_hw::Error;

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
        frame: Box<Frame<Command>>,
    },
    /// Communicate a unicast with an expected response.
    Communicate {
        /// The destination address.
        short_id: u16,
        /// The payload.
        command: Box<Command>,
        /// The response channel.
        response: Sender<Result<Receiver<Command>, Error>>,
    },
}
