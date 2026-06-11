//! Readable attributes of the Alarms cluster.

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use repr_discriminant::ReprDiscriminant;
use zigbee::types::Type;
use zigbee::{ClusterId, ClusterSpecific};

use crate::{InvalidType, ReadableAttribute};

impl ClusterSpecific for Id {
    const CLUSTER: ClusterId = ClusterId::Alarms;
}

/// Values of readable attributes for the Alarms cluster.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(u16)]
#[derive(ReprDiscriminant)]
pub enum Attribute {
    /// Number of alarms currently present in the alarm table.
    AlarmCount(u16) = 0x0000, // Valid range `0x00` to `0xff`.
}

impl TryFrom<(Id, Type)> for Attribute {
    type Error = InvalidType<Id>;

    fn try_from((id, typ): (Id, Type)) -> Result<Self, Self::Error> {
        match id {
            Id::AlarmCount => {
                if let Type::Uint16(value) = typ {
                    Ok(Self::AlarmCount(value.as_u16()))
                } else {
                    Err(InvalidType::new(id, typ))
                }
            }
        }
    }
}

impl ClusterSpecific for Attribute {
    const CLUSTER: ClusterId = ClusterId::Alarms;
}

/// Readable attributes for the Alarms cluster.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromPrimitive)]
#[repr(u16)]
pub enum Id {
    /// Number of alarms currently present in the alarm table.
    AlarmCount = 0x0000,
}

impl ReadableAttribute for Id {
    type Attribute = Attribute;
}

impl From<Id> for u16 {
    fn from(attr: Id) -> Self {
        attr as Self
    }
}

impl TryFrom<u16> for Id {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::from_u16(value).ok_or(value)
    }
}
