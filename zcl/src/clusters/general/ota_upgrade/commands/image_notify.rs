use zb_core::{Cluster, Direction};

use crate::macros::zcl_command;
use crate::ota_upgrade::ImageNotifyPayload;

zcl_command! {
    /// Notifies OTA clients that an upgrade image may be available.
    ///
    /// The generated command metadata disables default responses, which is the required setting
    /// for broadcast and multicast notifications. A unicast sender that needs a default response
    /// can construct its frame header with that bit cleared.
    ImageNotify {
        { Cluster::OtaUpgrade } => OtaUpgrade;
        command_id: 0x00;
        direction: Direction::ServerToClient;
        disable_default_response: true;
        fields {
            payload: ImageNotifyPayload,
        }

        getters {
            /// Return the notification payload.
            #[must_use]
            pub const fn payload(&self) -> ImageNotifyPayload {
                self.payload
            }
        }
    }
}
