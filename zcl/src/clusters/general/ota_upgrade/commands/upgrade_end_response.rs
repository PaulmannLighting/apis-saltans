use zb_core::{Cluster, Direction};

use crate::macros::zcl_command;
use crate::ota_upgrade::ImageId;

zcl_command! {
    /// Instructs an OTA client when to apply a downloaded image.
    UpgradeEndResponse {
        { Cluster::OtaUpgrade } => OtaUpgrade;
        command_id: 0x07;
        direction: Direction::ServerToClient;
        disable_default_response: true;
        derive(Copy);
        fields {
            image: ImageId,
            current_time: u32,
            upgrade_time: u32,
        }

        getters {
            /// Return the downloaded image identifier.
            #[must_use]
            pub const fn image(self) -> ImageId {
                self.image
            }

            /// Return the server's current time.
            #[must_use]
            pub const fn current_time(self) -> u32 {
                self.current_time
            }

            /// Return the time at which the client should apply the image.
            #[must_use]
            pub const fn upgrade_time(self) -> u32 {
                self.upgrade_time
            }
        }
    }
}
