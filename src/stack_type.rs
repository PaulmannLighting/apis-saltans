use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Eq, PartialEq, FromPrimitive, ToPrimitive)]
pub enum StackType {
    Zigbee2006 = 0x0000,
    Zigbee2007 = 0x0001,
    ZigbeePro = 0x0002,
    ZigbeeIp = 0x0003,
}

impl Display for StackType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Zigbee2006 => write!(f, "ZigBee 2006"),
            Self::Zigbee2007 => write!(f, "ZigBee 2007"),
            Self::ZigbeePro => write!(f, "ZigBee Pro"),
            Self::ZigbeeIp => write!(f, "ZigBee IP"),
        }
    }
}

impl From<StackType> for u16 {
    fn from(stack_type: StackType) -> Self {
        stack_type
            .to_u16()
            .expect("Could not convert StackType to u16")
    }
}
