//! Client-side attributes of the OTA Upgrade cluster.

use zb_core::Cluster;
use zb_core::types::{Uint16, Uint32};

pub use self::types::{ImageUpgradeStatus, UpgradeServerId};
use crate::macros::zcl_attributes;

mod types;

zcl_attributes! {
    cluster: Cluster::OtaUpgrade;

    /// IEEE address of the OTA upgrade server currently used by the client.
    UpgradeServerId = 0x0000: UpgradeServerId { R },
    /// Current byte offset in the OTA upgrade image.
    FileOffset = 0x0001: Uint32 { R },
    /// File version of the image currently running on the client.
    CurrentFileVersion = 0x0002: Uint32 { R },
    /// Zigbee stack version of the image currently running on the client.
    CurrentZigbeeStackVersion = 0x0003: Uint16 { R },
    /// File version of the image downloaded by the client.
    DownloadedFileVersion = 0x0004: Uint32 { R },
    /// Zigbee stack version of the image downloaded by the client.
    DownloadedZigbeeStackVersion = 0x0005: Uint16 { R },
    /// Current state of the client's image download and upgrade process.
    ImageUpgradeStatus = 0x0006: ImageUpgradeStatus { R },
    /// Zigbee-assigned manufacturer identifier of the client.
    ManufacturerId = 0x0007: Uint16 { R },
    /// Image type currently being downloaded or waiting to be applied.
    ImageTypeId = 0x0008: Uint16 { R },
    /// Minimum delay in seconds between image block requests.
    MinimumBlockPeriod = 0x0009: Uint16 { R },
    /// Build-specific stamp used as a second image identifier.
    ImageStamp = 0x000A: Uint32 { R },
}

#[cfg(test)]
mod tests {
    use zb_core::IeeeAddress;
    use zb_core::types::{Enum8, Type};

    use super::{Id, ImageUpgradeStatus, Readable, UpgradeServerId};

    const UPGRADE_SERVER: IeeeAddress = IeeeAddress::new(1, 2, 3, 4, 5, 6, 7, 8);

    #[test]
    fn parses_upgrade_server_attribute() {
        let attribute =
            Readable::try_from((Id::UpgradeServerId, Type::IeeeAddress(UPGRADE_SERVER)))
                .expect("valid upgrade server attribute");

        assert_eq!(
            attribute,
            Readable::UpgradeServerId(UpgradeServerId::new(UPGRADE_SERVER))
        );
    }

    #[test]
    fn parses_image_upgrade_status_attribute() {
        let status = ImageUpgradeStatus::DownloadInProgress;
        let attribute = Readable::try_from((
            Id::ImageUpgradeStatus,
            Type::Enum8(Enum8::new(status.into())),
        ))
        .expect("valid image upgrade status attribute");

        assert_eq!(attribute, Readable::ImageUpgradeStatus(status));
    }
}
