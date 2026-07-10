use tokio::sync::oneshot::{Receiver, Sender};
use zb_core::short_id::Device;
use zb_hw::Error;
use zb_nwk::Source;
use zb_zdp::{Command, Frame};

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
