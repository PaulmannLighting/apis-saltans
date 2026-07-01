//! Readable attributes of the Alarms cluster.

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use repr_discriminant::ReprDiscriminant;
use apis_saltans_core::types::Type;
use apis_saltans_core::{ClusterId, ClusterSpecific};

use super::AlarmCount;
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
    AlarmCount(AlarmCount) = 0x0000, // Valid range `0x00` to `0xff`.
}

impl From<Attribute> for Type {
    fn from(attribute: Attribute) -> Self {
        match attribute {
            Attribute::AlarmCount(count) => count.into(),
        }
    }
}

impl From<Attribute> for (u16, Type) {
    fn from(attribute: Attribute) -> Self {
        let id = attribute.discriminant();
        (id, attribute.into())
    }
}

impl TryFrom<(Id, Type)> for Attribute {
    type Error = InvalidType<Id>;

    fn try_from((id, typ): (Id, Type)) -> Result<Self, Self::Error> {
        match id {
            Id::AlarmCount => typ.try_into().map(Self::AlarmCount),
        }
        .map_err(|typ| InvalidType::new(id, typ))
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
