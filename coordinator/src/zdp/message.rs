use bytes::Bytes;
use tokio::sync::oneshot::Sender;
use zb_aps::apsde::{DataIndication, DataRequest};
use zb_core::short_id::Device;
use zb_zdp::{Command, Frame};

use crate::Error;
use crate::correlation::Token;
use crate::response::ApsProtocolResponse;

/// Messages exchanged with the transceiver actor.
#[derive(Debug)]
pub enum Message {
    /// A hardware-level event.
    Received {
        /// APSDE indication containing the parsed ZDP frame.
        indication: DataIndication<Frame<Command>, (), ()>,
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

    /// Report a failed deferred APS completion for a locally generated ZDP response.
    ReplyTransmissionFailed {
        /// Hardware failure reported after the response was queued.
        error: zb_hw::Error,
    },

    /// Retire a completed background ZDP server operation.
    ServerOperationFinished {
        /// Coordinator-private identity of the completed operation.
        id: u64,
    },

    /// Complete background handoff of a communicating request to the APS actor.
    CommunicationSubmissionFinished {
        /// Coordinator-private identity of the completed submission.
        id: u64,
        /// APS actor handoff result.
        result: Result<crate::aps::TransmissionResponse, Error>,
    },

    /// Communicate a unicast with an expected response.
    Communicate {
        /// Remote device expected to answer the request.
        device: Device,
        /// Complete APS data-service request.
        request: DataRequest<Bytes>,
        /// The response channel.
        response: Sender<Result<ApsProtocolResponse<Command>, Error>>,
    },
}
