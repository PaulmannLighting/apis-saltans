use tokio::sync::oneshot::Sender;
use zb_aps::Data;
use zb_core::Destination;
use zb_core::destination::Device;
use zb_hw::{Error, HwResponse};
use zb_nwk::Source;
use zb_zcl::{Cluster, Frame};

pub use super::Payload;
use crate::response::InternalCommunicationResponse;

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
        /// APS destination for the outgoing frame.
        destination: Destination,
        /// ZCL payload and its transmission metadata.
        payload: Payload,
        /// The response channel.
        response: Sender<Result<HwResponse, Error>>,
    },

    /// Communicate a unicast with an expected response.
    Communicate {
        /// Remote device expected to answer the command.
        device: Device,
        /// ZCL payload and its transmission metadata.
        payload: Payload,
        /// The response channel.
        response: Sender<Result<InternalCommunicationResponse<Cluster>, Error>>,
    },
}
