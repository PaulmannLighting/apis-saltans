//! Client-side attributes of the OTA Upgrade cluster.

use zb_core::Cluster;
use zb_core::types::{Uint16, Uint32};

pub use self::types::{ImageUpgradeStatus, UpgradeServerId};
use crate::macros::zcl_attributes;

mod types;

const UPGRADE_SERVER_ID_ATTRIBUTE_ID: u16 = 0x0000;
const FILE_OFFSET_ATTRIBUTE_ID: u16 = 0x0001;
const CURRENT_FILE_VERSION_ATTRIBUTE_ID: u16 = 0x0002;
const CURRENT_ZIGBEE_STACK_VERSION_ATTRIBUTE_ID: u16 = 0x0003;
const DOWNLOADED_FILE_VERSION_ATTRIBUTE_ID: u16 = 0x0004;
const DOWNLOADED_ZIGBEE_STACK_VERSION_ATTRIBUTE_ID: u16 = 0x0005;
const IMAGE_UPGRADE_STATUS_ATTRIBUTE_ID: u16 = 0x0006;
const MANUFACTURER_ID_ATTRIBUTE_ID: u16 = 0x0007;
const IMAGE_TYPE_ID_ATTRIBUTE_ID: u16 = 0x0008;
const MINIMUM_BLOCK_PERIOD_ATTRIBUTE_ID: u16 = 0x0009;
const IMAGE_STAMP_ATTRIBUTE_ID: u16 = 0x000A;

zcl_attributes! {
    cluster: Cluster::OtaUpgrade;

    /// IEEE address of the OTA upgrade server currently used by the client.
    UpgradeServerId = UPGRADE_SERVER_ID_ATTRIBUTE_ID: UpgradeServerId { R },
    /// Current byte offset in the OTA upgrade image.
    FileOffset = FILE_OFFSET_ATTRIBUTE_ID: Uint32 { R },
    /// File version of the image currently running on the client.
    CurrentFileVersion = CURRENT_FILE_VERSION_ATTRIBUTE_ID: Uint32 { R },
    /// Zigbee stack version of the image currently running on the client.
    CurrentZigbeeStackVersion = CURRENT_ZIGBEE_STACK_VERSION_ATTRIBUTE_ID: Uint16 { R },
    /// File version of the image downloaded by the client.
    DownloadedFileVersion = DOWNLOADED_FILE_VERSION_ATTRIBUTE_ID: Uint32 { R },
    /// Zigbee stack version of the image downloaded by the client.
    DownloadedZigbeeStackVersion = DOWNLOADED_ZIGBEE_STACK_VERSION_ATTRIBUTE_ID: Uint16 { R },
    /// Current state of the client's image download and upgrade process.
    ImageUpgradeStatus = IMAGE_UPGRADE_STATUS_ATTRIBUTE_ID: ImageUpgradeStatus { R },
    /// Zigbee-assigned manufacturer identifier of the client.
    ManufacturerId = MANUFACTURER_ID_ATTRIBUTE_ID: Uint16 { R },
    /// Image type currently being downloaded or waiting to be applied.
    ImageTypeId = IMAGE_TYPE_ID_ATTRIBUTE_ID: Uint16 { R },
    /// Minimum delay in seconds between image block requests.
    MinimumBlockPeriod = MINIMUM_BLOCK_PERIOD_ATTRIBUTE_ID: Uint16 { R },
    /// Build-specific stamp used as a second image identifier.
    ImageStamp = IMAGE_STAMP_ATTRIBUTE_ID: Uint32 { R },
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
