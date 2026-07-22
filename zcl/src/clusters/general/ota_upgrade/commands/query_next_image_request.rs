use std::vec::Vec;

use zb_core::{Cluster, Direction};

use crate::macros::zcl_command;
use crate::ota_upgrade::ImageId;

const HARDWARE_VERSION_PRESENT: u8 = 0b0000_0001;

zcl_command! {
    /// Queries the OTA server for the next image applicable to a client.
    QueryNextImageRequest {
        { Cluster::OtaUpgrade } => OtaUpgrade;
        command_id: 0x01;
        direction: Direction::ClientToServer;
        disable_default_response: false;
        response: super::QueryNextImageResponse;
        fields {
            image: ImageId,
            hardware_version: Option<u16>,
        }

        getters {
            /// Return the currently running image identifier.
            #[must_use]
            pub const fn image(&self) -> ImageId {
                self.image
            }

            /// Return the client's hardware version when supplied.
            #[must_use]
            pub const fn hardware_version(&self) -> Option<u16> {
                self.hardware_version
            }
        }

        from_le_stream {
            fn from_le_stream<T>(mut bytes: T) -> Option<Self>
            where
                T: Iterator<Item = u8>,
            {
                let field_control = u8::from_le_stream(&mut bytes)?;
                let image = ImageId::from_le_stream(&mut bytes)?;
                let hardware_version = if field_control & HARDWARE_VERSION_PRESENT == 0 {
                    None
                } else {
                    Some(u16::from_le_stream(bytes)?)
                };

                Some(Self {
                    image,
                    hardware_version,
                })
            }
        }

        to_le_stream {
            type Iter = <Vec<u8> as IntoIterator>::IntoIter;

            fn to_le_stream(self) -> Self::Iter {
                let field_control = self
                    .hardware_version
                    .map_or(0, |_| HARDWARE_VERSION_PRESENT);
                let mut bytes = Vec::new();
                bytes.push(field_control);
                bytes.extend(self.image.to_le_stream());
                bytes.extend(self.hardware_version.to_le_stream());
                bytes.into_iter()
            }
        }
    }
}
