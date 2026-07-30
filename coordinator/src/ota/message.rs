use thiserror::Error as ThisError;
use tokio::sync::oneshot;
use zb_aps::apsde::{DataIndication, IndividualEndpoint};
use zb_core::FullAddress;
use zb_zcl::Frame;
use zb_zcl::ota_upgrade::Command as OtaCommand;

use super::Image;

/// Terminal result delivered to the caller that scheduled an OTA update.
pub type UpdateResult = Result<(), UpdateError>;

/// Messages accepted by the coordinator OTA server.
#[derive(Debug)]
pub enum Message {
    /// Offer a validated OTA image to one device endpoint.
    Update {
        /// Complete IEEE and NWK address of the device to update.
        target: FullAddress,
        /// Remote OTA client endpoint.
        target_endpoint: IndividualEndpoint,
        /// Local OTA server endpoint used as the APS source.
        source_endpoint: IndividualEndpoint,
        /// Complete OTA image offered to the device.
        image: Image,
        /// Reports the terminal result of the scheduled update.
        completion: oneshot::Sender<UpdateResult>,
    },
    /// A received OTA Upgrade cluster command.
    Received {
        /// APSDE indication containing the typed OTA command and all receive metadata.
        indication: DataIndication<Frame<OtaCommand>, (), ()>,
    },
    /// Stop every active update because hardware events are unavailable.
    HardwareUnavailable,
}

/// Terminal failure reported by a coordinator-managed OTA update.
#[derive(Clone, Copy, Debug, Eq, PartialEq, ThisError)]
pub enum UpdateError {
    /// The OTA server could not register its ZCL subscription.
    #[error("the OTA ZCL subscription could not be registered")]
    Subscription,
    /// The configured number of concurrent destination OTA transfer tasks has been reached.
    #[error("the concurrent destination OTA transfer task limit of {limit} has been reached")]
    UpdateTaskLimitReached {
        /// Configured maximum number of concurrent destination OTA transfer tasks.
        limit: usize,
    },
    /// A destination transfer task stopped unexpectedly.
    #[error("the OTA transfer task stopped unexpectedly")]
    TransferTask,
    /// A newer image replaced this update for the same device endpoint.
    #[error("the OTA update was superseded by a newer image")]
    Superseded,
    /// The OTA client aborted the update.
    #[error("the OTA client aborted the update")]
    Aborted,
    /// The OTA client rejected the downloaded image as invalid.
    #[error("the OTA client rejected the downloaded image")]
    InvalidImage,
    /// The OTA client requires another image before it can upgrade.
    #[error("the OTA client requires another image")]
    RequireMoreImage,
    /// Reading image data failed.
    #[error("reading OTA image data failed")]
    ImageTransfer,
    /// Transmitting an OTA command failed.
    #[error("transmitting OTA data failed")]
    Transmission,
    /// The hardware event stream closed before the OTA update completed.
    #[error("the hardware event stream closed")]
    HardwareEventStreamClosed,
}
