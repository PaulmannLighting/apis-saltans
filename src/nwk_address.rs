use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NwkAddress {
    Group(u16),
    Device(u16),
}

impl Display for NwkAddress {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Group(address) => write!(f, "Group({address})"),
            Self::Device(address) => write!(f, "Device({address})"),
        }
    }
}

impl From<NwkAddress> for u8 {
    fn from(nwk_address: NwkAddress) -> Self {
        match nwk_address {
            NwkAddress::Group(_) => 0x01,
            NwkAddress::Device(_) => 0x02,
        }
    }
}
