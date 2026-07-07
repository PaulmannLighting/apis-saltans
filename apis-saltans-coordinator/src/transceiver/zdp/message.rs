use apis_saltans_hw::Error;
use apis_saltans_nwk::Source;
use apis_saltans_zdp::{Command, Frame};
use tokio::sync::oneshot::{Receiver, Sender};

pub use self::payload::Payload;

mod payload;

/// Messages exchanged with the transceiver actor.
#[derive(Debug)]
pub enum Message {
    /// A hardware-level event.
    Received {
        /// The source information of the frame.
        source: Source,
        /// The APS frame.
        frame: Frame<Command>,
    },
    /// Communicate a unicast with an expected response.
    Communicate {
        /// The destination address.
        short_id: u16,
        /// The payload.
        command: Command,
        /// The response channel.
        response: Sender<Result<Receiver<Command>, Error>>,
    },
}
