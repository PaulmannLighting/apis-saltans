use apis_saltans_core::short_id::Device;
use apis_saltans_hw::Error;
use apis_saltans_nwk::Source;
use apis_saltans_zdp::{Command, Frame};
use tokio::sync::oneshot::{Receiver, Sender};

use super::Payload;

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
        device: Device,
        /// The payload.
        payload: Payload,
        /// The response channel.
        response: Sender<Result<Receiver<Command>, Error>>,
    },
}
