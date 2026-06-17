use tokio::sync::oneshot::Sender;

use super::error::Error;
use crate::network_manager::Device;

/// Persistence messages.
pub enum Message {
    /// Save devices.
    Save {
        /// Devices to save.
        state: Box<[Device]>,

        /// Response sender.
        response: Sender<Result<(), Error>>,
    },

    /// Load devices.
    Load {
        /// Response sender.
        response: Sender<Result<Box<[Device]>, Error>>,
    },
}
