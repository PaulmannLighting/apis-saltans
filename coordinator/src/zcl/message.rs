use bytes::Bytes;
use tokio::sync::mpsc::Sender as MpscSender;
use tokio::sync::oneshot::Sender;
use zb_aps::apsde::{DataIndication, DataRequest};
use zb_zcl::{Cluster, UnsequencedFrame};

use super::{Subscription, SubscriptionMessage};
use crate::aps::TransmissionResponse;
use crate::correlation::Token;
use crate::response::ApsProtocolResponse;
use crate::{Error, RawExpectedPacket};

/// Messages exchanged with the transceiver actor.
#[derive(Debug)]
pub enum Message {
    /// Register a filtered receiver for incoming ZCL frames.
    Subscribe {
        /// Subscription to register.
        subscription: Subscription,
    },

    /// Remove a registered subscription.
    Unsubscribe {
        /// Sending handle identifying the subscription channel to remove.
        messages: MpscSender<SubscriptionMessage>,
    },

    /// A hardware-level event.
    Received {
        /// Normalized APSDE indication containing the original complete ZCL ASDU.
        indication: DataIndication<Bytes, (), ()>,
    },

    /// Fail pending protocol responses because the Zigbee network went down.
    NetworkDown,

    /// Fail pending protocol responses and stop because hardware events are unavailable.
    HardwareUnavailable,

    /// Cancel a pending protocol response whose future was dropped.
    Cancel {
        /// Coordinator-private identity of the protocol transaction to cancel.
        token: Token,
    },

    /// Expire a pending protocol response.
    ResponseTimeout {
        /// Coordinator-private identity whose response timeout elapsed.
        token: Token,
    },

    /// Release a response identity after its late-response grace period.
    QuarantineTimeout {
        /// Coordinator-private identity whose quarantine timeout elapsed.
        token: Token,
    },

    /// Unicast a message.
    Transmit {
        /// APS request containing the outgoing ZCL command.
        request: DataRequest<UnsequencedFrame<Bytes>>,
        /// Channel used to return the deferred APS transmission result.
        response: Sender<Result<TransmissionResponse, Error>>,
    },

    /// Reply to a received command using its ZCL sequence number.
    Reply {
        /// Sequence number copied from the request, or advanced for a page response stream.
        sequence_number: u8,
        /// APS request containing the outgoing ZCL reply.
        request: DataRequest<UnsequencedFrame<Bytes>>,
        /// Channel used to return the deferred APS transmission result.
        response: Sender<Result<TransmissionResponse, Error>>,
    },

    /// Communicate a unicast with an expected response.
    Communicate {
        /// APS request containing the outgoing ZCL command.
        request: DataRequest<UnsequencedFrame<Bytes>>,
        /// The response channel.
        response: Sender<Result<ApsProtocolResponse<Cluster>, Error>>,
    },

    /// Communicate a unicast with one explicitly selected raw response.
    CommunicateRaw {
        /// APS request containing the outgoing ZCL command.
        request: DataRequest<UnsequencedFrame<Bytes>>,
        /// Command scope and ID required for raw completion.
        expected_response: RawExpectedPacket,
        /// Channel returning the deferred native APS and protocol response.
        response: Sender<Result<ApsProtocolResponse<Bytes>, Error>>,
    },
}
