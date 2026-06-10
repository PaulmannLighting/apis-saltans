//! Readable attributes for the On/Off cluster.

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use repr_discriminant::ReprDiscriminant;
use zigbee::types::{Bool, Type, Uint16};
use zigbee::{ClusterId, ClusterSpecific};

use super::StartUpOnOff;
use crate::{InvalidType, ReadableAttribute};

/// Readable attributes for the On/Off cluster.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(u16)]
#[derive(ReprDiscriminant)]
pub enum Attribute {
    /// On/Off state of the device.
    OnOff(Bool) = 0x0000,
    ///  Global scene control.
    GlobalSceneControl(Bool) = 0x4000,
    ///  On time attribute.
    OnTime(Uint16) = 0x4001,
    ///  Off wait time attribute.
    OffWaitTime(Uint16) = 0x4002,
    ///  Start up on off attribute.
    StartUpOnOff(StartUpOnOff) = 0x4003,
}

impl TryFrom<(Id, Type)> for Attribute {
    type Error = InvalidType<Id>;

    fn try_from((id, typ): (Id, Type)) -> Result<Self, Self::Error> {
        match id {
            Id::OnOff => {
                if let Type::Boolean(on_off) = typ {
                    Ok(Self::OnOff(on_off))
                } else {
                    Err(InvalidType::new(id, typ))
                }
            }
            Id::GlobalSceneControl => {
                if let Type::Boolean(on_off) = typ {
                    Ok(Self::GlobalSceneControl(on_off))
                } else {
                    Err(InvalidType::new(id, typ))
                }
            }
            Id::OnTime => {
                if let Type::Uint16(on_time) = typ {
                    Ok(Self::OnTime(on_time))
                } else {
                    Err(InvalidType::new(id, typ))
                }
            }
            Id::OffWaitTime => {
                if let Type::Uint16(off_time) = typ {
                    Ok(Self::OffWaitTime(off_time))
                } else {
                    Err(InvalidType::new(id, typ))
                }
            }
            Id::StartUpOnOff => {
                if let Type::Map8(value) = typ
                    && let Ok(startup_on_off) = value.try_into()
                {
                    Ok(Self::StartUpOnOff(startup_on_off))
                } else {
                    Err(InvalidType::new(id, typ))
                }
            }
        }
    }
}

/// Readable attribute IDs for the On/Off cluster.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, FromPrimitive)]
#[repr(u16)]
pub enum Id {
    /// On/Off state of the device.
    OnOff = 0x0000,
    ///  Global scene control.
    GlobalSceneControl = 0x4000,
    ///  On time attribute.
    OnTime = 0x4001,
    ///  Off wait time attribute.
    OffWaitTime = 0x4002,
    ///  Start up on off attribute.
    StartUpOnOff = 0x4003,
}

impl ClusterSpecific for Id {
    const CLUSTER: ClusterId = ClusterId::OnOff;
}

impl ReadableAttribute for Id {
    type Attribute = Attribute;
}

impl From<Id> for u16 {
    fn from(id: Id) -> Self {
        id as u16
    }
}

impl TryFrom<u16> for Id {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::from_u16(value).ok_or(value)
    }
}
