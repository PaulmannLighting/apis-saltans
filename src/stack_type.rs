use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Eq, PartialEq)]
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
