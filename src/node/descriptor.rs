use serde::{Deserialize, Serialize};

use flags::Flags;
use mac_capability_flags::MacCapabilityFlags;

mod device_type;
mod flags;
mod frequency_band;
mod logical_type;
mod mac_capability_flags;
mod server_mask;

#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct Descriptor {
    flags: Flags,
    mac_capability_flags: MacCapabilityFlags,
    manufacturer_code: u16,
    maximum_buffer_size: u8,
    maximum_incoming_transfer_size: u16,
    server_mask: u16,
    maximum_outgoing_transfer_size: u16,
    descriptor_capability: u8,
}
