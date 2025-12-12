use std::fmt::Display;

use macaddr::MacAddr8;
use repr_discriminant::ReprDiscriminant;

/// Address type for Bind Request.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, ReprDiscriminant)]
#[repr(u8)]
pub enum Destination {
    /// 16-bit group address.
    Group(u16) = 0x01,
    /// 64-bit extended address.
    Extended {
        /// 64-bit MAC address.
        address: MacAddr8,
        /// Endpoint on the destination device.
        endpoint: u8,
    } = 0x03,
}

impl Display for Destination {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Group(address) => write!(f, "Group {{ {address:#06X} }}"),
            Self::Extended { address, endpoint } => {
                write!(
                    f,
                    "Extended {{ address: {address}, endpoint: {endpoint:#04X} }}"
                )
            }
        }
    }
}
