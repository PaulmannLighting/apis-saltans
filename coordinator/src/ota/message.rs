use thiserror::Error as ThisError;
use tokio::sync::oneshot;
use zb_aps::Data;
use zb_core::Profile;
use zb_core::destination::Device;
use zb_nwk::Source;
use zb_zcl::Frame;
use zb_zcl::ota_upgrade::Command as OtaCommand;

use super::Image;

/// Terminal result delivered to the caller that scheduled an OTA update.
pub type UpdateResult = Result<(), UpdateError>;

/// A device endpoint and application profile targeted for an OTA update.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Target {
    destination: Device,
    profile: Profile,
}

impl Target {
    /// Create an OTA update target.
    #[must_use]
    pub const fn new(destination: Device, profile: Profile) -> Self {
        Self {
            destination,
            profile,
        }
    }

    /// Return the target device endpoint.
    #[must_use]
    pub const fn destination(self) -> Device {
        self.destination
    }

    /// Return the target application profile.
    #[must_use]
    pub const fn profile(self) -> Profile {
        self.profile
    }
}

/// Messages accepted by the coordinator OTA server.
#[derive(Debug)]
pub enum Message {
    /// Offer a validated OTA image to one device endpoint.
    Update {
        /// Device endpoint and profile to update.
        target: Target,
        /// Complete OTA image offered to the device.
        image: Image,
        /// Reports the terminal result of the scheduled update.
        completion: oneshot::Sender<UpdateResult>,
    },
    /// A received OTA Upgrade cluster command.
    Received {
        /// NWK source information supplied by the hardware backend.
        source: Source,
        /// Typed APS and ZCL frame.
        frame: Data<Frame<OtaCommand>>,
    },
}

/// Terminal failure reported by a coordinator-managed OTA update.
#[derive(Clone, Copy, Debug, Eq, PartialEq, ThisError)]
pub enum UpdateError {
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
}
