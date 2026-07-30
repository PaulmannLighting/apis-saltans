use bytes::Bytes;
use tokio::sync::oneshot::Sender;
use zb_aps::apsde::{DataIndication, DataRequest};
use zb_core::short_id::Device;
use zb_zdp::{Command, Frame};

use crate::Error;
use crate::index::Index;
use crate::response::ApsProtocolResponse;

/// Messages exchanged with the transceiver actor.
#[derive(Debug)]
pub enum Message {
    /// A hardware-level event.
    Received {
        /// APSDE indication containing the parsed ZDP frame.
        indication: DataIndication<Frame<Command>, (), ()>,
    },

    /// The network has been opened for new joins.
    NetworkOpened,

    /// The network has been closed for new joins.
    NetworkClosed,

    /// Fail pending protocol responses because the Zigbee network went down.
    NetworkDown,

    /// Cancel a pending protocol response whose future was dropped.
    Cancel {
        /// Correlation key to cancel.
        index: Index,
    },

    /// Expire a pending protocol response.
    ResponseTimeout {
        /// Correlation key whose timeout elapsed.
        index: Index,
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
