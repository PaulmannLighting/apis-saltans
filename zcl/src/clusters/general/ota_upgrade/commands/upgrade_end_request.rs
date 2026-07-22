use zb_core::{Cluster, Direction};

use crate::macros::zcl_command;
use crate::ota_upgrade::{ImageId, UpgradeEndStatus};

const COMMAND_ID: u8 = 0x06;

zcl_command! {
    /// Reports completion, validation failure, or termination of an OTA image download.
    UpgradeEndRequest {
        { Cluster::OtaUpgrade } => OtaUpgrade;
        command_id: COMMAND_ID;
        direction: Direction::ClientToServer;
        disable_default_response: false;
        response: super::UpgradeEndResponse;
        derive(Copy);
        fields {
            status: UpgradeEndStatus,
            image: ImageId,
        }

        getters {
            /// Return the outcome of the image download.
            #[must_use]
            pub const fn status(self) -> UpgradeEndStatus {
                self.status
            }

            /// Return the downloaded image identifier.
            #[must_use]
            pub const fn image(self) -> ImageId {
                self.image
            }
        }
    }
}
