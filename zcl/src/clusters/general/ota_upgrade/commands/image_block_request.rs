use std::vec::Vec;

use zb_core::{Cluster, Direction, IeeeAddress};

use crate::macros::zcl_command;
use crate::ota_upgrade::ImageId;

const REQUEST_NODE_ADDRESS_PRESENT: u8 = 0b0000_0001;
const MINIMUM_BLOCK_PERIOD_PRESENT: u8 = 0b0000_0010;

zcl_command! {
    /// Requests one block of an OTA image.
    ImageBlockRequest {
        { Cluster::OtaUpgrade } => OtaUpgrade;
        command_id: 0x03;
        direction: Direction::ClientToServer;
        disable_default_response: false;
        response: super::ImageBlockResponse;
        fields {
            image: ImageId,
            file_offset: u32,
            maximum_data_size: u8,
            request_node_address: Option<IeeeAddress>,
            minimum_block_period: Option<u16>,
        }

        getters {
            /// Return the requested image identifier.
            #[must_use]
            pub const fn image(&self) -> ImageId {
                self.image
            }

            /// Return the requested byte offset in the OTA file.
            #[must_use]
            pub const fn file_offset(&self) -> u32 {
                self.file_offset
            }

            /// Return the largest data block the client can receive.
            #[must_use]
            pub const fn maximum_data_size(&self) -> u8 {
                self.maximum_data_size
            }

            /// Return the requesting node's IEEE address when present.
            #[must_use]
            pub const fn request_node_address(&self) -> Option<IeeeAddress> {
                self.request_node_address
            }

            /// Return the client's current minimum block period in seconds when present.
            #[must_use]
            pub const fn minimum_block_period(&self) -> Option<u16> {
                self.minimum_block_period
            }
        }

        from_le_stream {
            fn from_le_stream<T>(mut bytes: T) -> Option<Self>
            where
                T: Iterator<Item = u8>,
            {
                let field_control = u8::from_le_stream(&mut bytes)?;
                let image = ImageId::from_le_stream(&mut bytes)?;
                let file_offset = u32::from_le_stream(&mut bytes)?;
                let maximum_data_size = u8::from_le_stream(&mut bytes)?;
                let request_node_address = if field_control & REQUEST_NODE_ADDRESS_PRESENT == 0 {
                    None
                } else {
                    Some(IeeeAddress::from_le_stream(&mut bytes)?)
                };
                let minimum_block_period = if field_control & MINIMUM_BLOCK_PERIOD_PRESENT == 0 {
                    None
                } else {
                    Some(u16::from_le_stream(bytes)?)
                };

                Some(Self {
                    image,
                    file_offset,
                    maximum_data_size,
                    request_node_address,
                    minimum_block_period,
                })
            }
        }

        to_le_stream {
            type Iter = <Vec<u8> as IntoIterator>::IntoIter;

            fn to_le_stream(self) -> Self::Iter {
                let mut field_control = 0;
                if self.request_node_address.is_some() {
                    field_control |= REQUEST_NODE_ADDRESS_PRESENT;
                }
                if self.minimum_block_period.is_some() {
                    field_control |= MINIMUM_BLOCK_PERIOD_PRESENT;
                }

                let mut bytes = Vec::new();
                bytes.push(field_control);
                bytes.extend(self.image.to_le_stream());
                bytes.extend(self.file_offset.to_le_stream());
                bytes.push(self.maximum_data_size);
                bytes.extend(self.request_node_address.to_le_stream());
                bytes.extend(self.minimum_block_period.to_le_stream());
                bytes.into_iter()
            }
        }
    }
}
