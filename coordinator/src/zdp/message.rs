use tokio::sync::oneshot::Sender;
use zb_aps::Data;
use zb_core::short_id::Device;
use zb_hw::Error;
use zb_nwk::Source;
use zb_zdp::{Command, Frame};

use super::Payload;
use crate::response::InternalCommunicationResponse;

/// Messages exchanged with the transceiver actor.
#[derive(Debug)]
pub enum Message {
    /// A hardware-level event.
    Received {
        /// The source information of the frame.
        source: Source,
        /// The APS frame.
        frame: Data<Frame<Command>>,
    },

    /// The network has been opened for new joins.
    NetworkOpened,

    /// The network has been closed for new joins.
    NetworkClosed,

    /// Communicate a unicast with an expected response.
    Communicate {
        device: Device,
        /// The payload.
        payload: Payload,
        /// The response channel.
        response: Sender<Result<InternalCommunicationResponse<Command>, Error>>,
    },
}
