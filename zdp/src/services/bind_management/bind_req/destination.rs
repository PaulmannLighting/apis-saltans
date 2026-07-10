use std::fmt::Display;

use apis_saltans_core::{Endpoint, IeeeAddress};
use repr_discriminant::ReprDiscriminant;

/// Address type for Bind Request.
#[cfg_attr(target_pointer_width = "64", expect(variant_size_differences))]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, ReprDiscriminant)]
#[repr(u8)]
pub enum Destination {
    /// 16-bit group address.
    Group(u16) = 0x01,

    /// 64-bit extended address.
    Extended {
        /// 64-bit MAC address.
        address: IeeeAddress,
        /// Endpoint on the destination device.
        endpoint: Endpoint,
    } = 0x03,
}

impl Display for Destination {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Group(address) => write!(f, "Group: {address:#06X} }}"),
            Self::Extended { address, endpoint } => {
                write!(f, "Extended: {address}:{endpoint}")
            }
        }
    }
}
