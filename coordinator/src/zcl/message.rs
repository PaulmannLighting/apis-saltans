use tokio::sync::oneshot::Sender;
use zb_aps::Data;
use zb_core::Destination;
use zb_core::destination::Device;
use zb_hw::Error;
use zb_nwk::Source;
use zb_zcl::{Cluster, Frame};

pub use super::Payload;
use super::Subscription;
use crate::aps::TransmissionResponse;
use crate::response::ApsProtocolResponse;

/// Messages exchanged with the transceiver actor.
#[derive(Debug)]
pub enum Message {
    /// Register a filtered receiver for incoming ZCL frames.
    Subscribe {
        /// Subscription to register.
        subscription: Subscription,
    },

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
        /// Channel used to return the deferred APS transmission result.
        response: Sender<Result<TransmissionResponse, Error>>,
    },

    /// Reply to a received command using its ZCL sequence number.
    Reply {
        /// Device endpoint to which the reply is sent.
        destination: Device,
        /// Sequence number copied from the request, or advanced for a page response stream.
        sequence_number: u8,
        /// ZCL payload and its transmission metadata.
        payload: Payload,
        /// Channel used to return the deferred APS transmission result.
        response: Sender<Result<TransmissionResponse, Error>>,
    },

    /// Communicate a unicast with an expected response.
    Communicate {
        /// Remote device expected to answer the command.
        device: Device,
        /// ZCL payload and its transmission metadata.
        payload: Payload,
        /// The response channel.
        response: Sender<Result<ApsProtocolResponse<Cluster>, Error>>,
    },
}
