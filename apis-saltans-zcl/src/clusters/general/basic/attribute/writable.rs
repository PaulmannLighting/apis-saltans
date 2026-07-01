//! Writable attributes in the Basic cluster.

use core::iter::Chain;

use le_stream::ToLeStream;
use repr_discriminant::ReprDiscriminant;
use apis_saltans_core::types::{Bool, String};
use apis_saltans_core::{ClusterId, ClusterSpecific};

use self::iterator::LeStreamIter;
use super::alarm_mask::AlarmMask;
use super::disable_local_config::DisableLocalConfig;
use super::physical_environment::PhysicalEnvironment;
use super::readable;
use crate::WritableAttribute;
use crate::global::write_attributes::Record;

mod iterator;

/// Writable attributes in the Basic cluster.
#[cfg_attr(target_pointer_width = "64", expect(variant_size_differences))]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u16)]
#[derive(ReprDiscriminant)]
pub enum Attribute {
    /// The generic device class.
    LocationDescription(Box<String<16>>) = 0x0010,

    /// The physical environment.
    PhysicalEnvironment(PhysicalEnvironment) = 0x0011,

    /// The device enabled state.
    DeviceEnabled(Bool) = 0x0012,

    /// The alarm mask.
    AlarmMask(AlarmMask) = 0x0013,

    /// Flags to disable local configuration.
    DisableLocalConfig(DisableLocalConfig) = 0x0014,
}

impl ClusterSpecific for Attribute {
    const CLUSTER: ClusterId = ClusterId::Basic;
}

impl WritableAttribute for Attribute {
    fn id(&self) -> u16 {
        self.discriminant()
    }
}

impl From<Attribute> for Record {
    fn from(attribute: Attribute) -> Self {
        let id = attribute.discriminant();

        match attribute {
            Attribute::LocationDescription(string) => Self::new(id, (*string).into()),
            Attribute::PhysicalEnvironment(physical_environment) => {
                Self::new(id, physical_environment.into())
            }
            Attribute::DeviceEnabled(device_enabled) => Self::new(id, device_enabled.into()),
            Attribute::AlarmMask(alarm_mask) => Self::new(id, alarm_mask.into()),
            Attribute::DisableLocalConfig(disable_local_config) => {
                Self::new(id, disable_local_config.into())
            }
        }
    }
}

impl TryFrom<readable::Attribute> for Attribute {
    type Error = readable::Attribute;

    fn try_from(value: readable::Attribute) -> Result<Self, Self::Error> {
        match value {
            readable::Attribute::LocationDescription(string) => {
                Ok(Self::LocationDescription(string.into()))
            }
            readable::Attribute::PhysicalEnvironment(physical_environment) => {
                Ok(Self::PhysicalEnvironment(physical_environment))
            }
            readable::Attribute::DeviceEnabled(device_enabled) => {
                Ok(Self::DeviceEnabled(device_enabled))
            }
            readable::Attribute::AlarmMask(mask) => Ok(Self::AlarmMask(mask)),
            readable::Attribute::DisableLocalConfig(value) => Ok(Self::DisableLocalConfig(value)),
            other => Err(other),
        }
    }
}

impl ToLeStream for Attribute {
    type Iter = Chain<<u16 as ToLeStream>::Iter, LeStreamIter>;

    fn to_le_stream(self) -> Self::Iter {
        self.discriminant()
            .to_le_stream()
            .chain(LeStreamIter::from(self))
    }
}
