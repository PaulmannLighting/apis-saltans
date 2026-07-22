use zb_core::{Cluster, Direction};

use crate::macros::zcl_command;
use crate::ota_upgrade::ImageBlockResponsePayload;

const COMMAND_ID: u8 = 0x05;

zcl_command! {
    /// Returns image data, wait timing, or an abort indication to an OTA client.
    ImageBlockResponse {
        { Cluster::OtaUpgrade } => OtaUpgrade;
        command_id: COMMAND_ID;
        direction: Direction::ServerToClient;
        disable_default_response: true;
        fields {
            payload: ImageBlockResponsePayload,
        }

        getters {
            /// Return the block response payload.
            #[must_use]
            pub const fn payload(&self) -> &ImageBlockResponsePayload {
                &self.payload
            }
        }
    }
}
