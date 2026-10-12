//! Type-directed selection of attribute reporting parameters.

use std::fmt::Debug;
use std::hash::Hash;

use le_stream::ToLeStream;
use zb_core::types::{
    Bool, Date, Int8, Int16, Int24, Int32, Int40, Int48, Int56, Int64, OctStr, String, TimeOfDay,
    Uint8, Uint16, Uint24, Uint32, Uint40, Uint48, Uint56, Uint64, UtcTime,
};
use zb_core::units::Mireds;
use zb_core::{IeeeAddress, TypeId};

use super::{Analog, Discrete};
use crate::global::configure_reporting::send::AttributeReportingConfiguration;

/// Reporting parameters associated with an attribute value type.
///
/// Aliases inherit their underlying type's reporting behavior. New attribute types must
/// explicitly choose their parameters instead of relying on their spelling in a macro.
pub trait ReportingType: TypeId {
    /// Parameters used to configure this attribute's reports.
    type Configuration: ReportingConfiguration + Clone + Debug + Eq + Ord + Hash;
}

/// Convert typed reporting parameters into a wire configuration record.
pub trait ReportingConfiguration {
    /// Build the record for the supplied attribute identifier.
    fn into_configuration(self, attribute_id: u16) -> AttributeReportingConfiguration;
}

impl<T> ReportingConfiguration for Analog<T>
where
    T: TypeId + ToLeStream,
{
    fn into_configuration(self, attribute_id: u16) -> AttributeReportingConfiguration {
        AttributeReportingConfiguration::analog(attribute_id, self)
    }
}

impl<T> ReportingConfiguration for Discrete<T>
where
    T: TypeId,
{
    fn into_configuration(self, attribute_id: u16) -> AttributeReportingConfiguration {
        AttributeReportingConfiguration::discrete(attribute_id, &self)
    }
}

macro_rules! analog_types {
    ($($ty:ty),* $(,)?) => { $(impl ReportingType for $ty {
        type Configuration = Analog<Self>;
    })* };
}

analog_types!(
    Uint8, Uint16, Uint24, Uint32, Uint40, Uint48, Uint56, Uint64, Int8, Int16, Int24, Int32,
    Int40, Int48, Int56, Int64, Date, TimeOfDay, UtcTime
);

impl ReportingType for Bool {
    type Configuration = Discrete<Self>;
}

impl ReportingType for IeeeAddress {
    type Configuration = Discrete<Self>;
}
impl ReportingType for Mireds {
    type Configuration = Analog<Self>;
}
impl<const N: usize> ReportingType for String<N> {
    type Configuration = Discrete<Self>;
}
impl<const N: usize> ReportingType for OctStr<N> {
    type Configuration = Discrete<Self>;
}
