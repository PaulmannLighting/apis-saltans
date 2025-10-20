//! Writable attributes for the Device Temperature Configuration cluster.

use core::iter::Chain;

use le_stream::ToLeStream;
use le_stream::derive::FromLeStreamTagged;
use repr_discriminant::ReprDiscriminant;

use super::iterator;
use crate::types::Uint24;
use crate::zcl::device_temperature_configuration::{DeviceTempAlarmMask, Temperature};

/// Writable attributes for the Device Temperature Configuration cluster.
///
/// Those are equivalent to the temperature settings.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u16)]
#[derive(ReprDiscriminant, FromLeStreamTagged)]
pub enum Attribute {
    /// Alarms mask for device temperature.
    DeviceTempAlarmMask(DeviceTempAlarmMask) = 0x0010,
    /// Low temperature threshold in degrees Celsius.
    LowTempThreshold(Temperature) = 0x0011,
    /// High temperature threshold in degrees Celsius.
    HighTempThreshold(Temperature) = 0x0012,
    /// Low temperature dwell trip point in seconds.
    LowTempDwellTripPoint(Uint24) = 0x0013,
    /// High temperature dwell trip point in seconds.
    HighTempDwellTripPoint(Uint24) = 0x0014,
}

impl ToLeStream for Attribute {
    type Iter = Chain<<u16 as ToLeStream>::Iter, iterator::Attribute>;

    fn to_le_stream(self) -> Self::Iter {
        let id = self.discriminant();
        let payload_iter: iterator::Attribute = match self {
            Self::LowTempThreshold(thresh) | Self::HighTempThreshold(thresh) => thresh.into(),
            Self::DeviceTempAlarmMask(mask) => mask.into(),
            Self::LowTempDwellTripPoint(seconds) | Self::HighTempDwellTripPoint(seconds) => {
                seconds.into()
            }
        };
        id.to_le_stream().chain(payload_iter)
    }
}
