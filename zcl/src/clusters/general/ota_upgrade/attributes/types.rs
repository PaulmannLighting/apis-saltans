//! Attribute value types of the OTA Upgrade cluster.

use zb_core::IeeeAddress;

use crate::macros::zcl_attribute_newtype;

zcl_attribute_newtype! {
    /// IEEE address of the OTA upgrade server.
    pub struct UpgradeServerId(IeeeAddress) => IeeeAddress;
}

zcl_attribute_newtype! {
    /// State of the OTA image download and upgrade process.
    pub enum ImageUpgradeStatus: Enum8 {
        /// No download or upgrade is in progress.
        Normal = 0x00,
        /// An image download is in progress.
        DownloadInProgress = 0x01,
        /// The image download and integrity checks are complete.
        DownloadComplete = 0x02,
        /// The client is waiting for the server to initiate the upgrade.
        WaitingToUpgrade = 0x03,
        /// The client is counting down to the upgrade time.
        CountDown = 0x04,
        /// The client is waiting for another required image.
        WaitForMore = 0x05,
    }
}
