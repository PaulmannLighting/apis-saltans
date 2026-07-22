use zb_core::{Cluster, Direction, IeeeAddress};

use crate::macros::zcl_command;
use crate::ota_upgrade::ImageId;

zcl_command! {
    /// Queries the OTA server for a file specific to one client device.
    QuerySpecificFileRequest {
        { Cluster::OtaUpgrade } => OtaUpgrade;
        command_id: 0x08;
        direction: Direction::ClientToServer;
        disable_default_response: false;
        response: super::QuerySpecificFileResponse;
        derive(Copy);
        fields {
            request_node_address: IeeeAddress,
            image: ImageId,
            zigbee_stack_version: u16,
        }

        getters {
            /// Return the IEEE address of the requesting client.
            #[must_use]
            pub const fn request_node_address(self) -> IeeeAddress {
                self.request_node_address
            }

            /// Return the requested file identifier.
            #[must_use]
            pub const fn image(self) -> ImageId {
                self.image
            }

            /// Return the Zigbee stack version relevant to the requested file.
            #[must_use]
            pub const fn zigbee_stack_version(self) -> u16 {
                self.zigbee_stack_version
            }
        }
    }
}
