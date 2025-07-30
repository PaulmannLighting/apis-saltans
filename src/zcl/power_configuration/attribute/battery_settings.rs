use le_stream::FromLeStream;
use repr_discriminant::ReprDiscriminant;

use crate::types::{String16, Uint8, Uint16};
use crate::zcl::power_configuration::attribute::{
    BatteryAlarmMask, BatteryAlarmState, BatterySize,
};

/// Available battery settings.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u16)]
#[derive(ReprDiscriminant)]
pub enum BatterySettings {
    /// Name of the battery manufacturer.
    BatteryManufacturer(String16) = 0x0000,
    /// The battery size.
    BatterySize(BatterySize) = 0x0001,
    /// The battery ampere-hour rating in 10mAHr.
    BatteryAHrRating(Uint16) = 0x0002,
    /// Amount of battery cells.
    BatteryQuantity(Uint8) = 0x0003,
    /// The battery rated voltage in 100mV.
    BatteryRatedVoltage(Uint8) = 0x0004,
    /// The battery alarm mask.
    BatteryAlarmMask(BatteryAlarmMask) = 0x0005,
    /// The minimum battery voltage threshold in 100mV.
    BatteryVoltageMinThreshold(Uint8) = 0x0006,
    /// The first battery voltage threshold in 100mV.
    BatteryVoltageThreshold1(Uint8) = 0x0007,
    /// The second battery voltage threshold in 100mV.
    BatteryVoltageThreshold2(Uint8) = 0x0008,
    /// The third battery voltage threshold in 100mV.
    BatteryVoltageThreshold3(Uint8) = 0x0009,
    /// The minimum battery percentage threshold.
    BatteryPercentageMinThreshold(Uint8) = 0x000a,
    /// The first battery percentage threshold.
    BatteryPercentageThreshold1(Uint8) = 0x000b,
    /// The second battery percentage threshold.
    BatteryPercentageThreshold2(Uint8) = 0x000c,
    /// The third battery percentage threshold.
    BatteryPercentageThreshold3(Uint8) = 0x000d,
    /// The battery alarm state.
    BatteryAlarmState(BatteryAlarmState) = 0x000e,
}

impl BatterySettings {
    pub(crate) fn from_le_stream<T>(mask: u16, bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        match mask {
            0x0000 => String16::from_le_stream(bytes).map(Self::BatteryManufacturer),
            0x0001 => BatterySize::from_le_stream(bytes).map(Self::BatterySize),
            0x0002 => Uint16::from_le_stream(bytes).map(Self::BatteryAHrRating),
            0x0003 => Uint8::from_le_stream(bytes).map(Self::BatteryQuantity),
            0x0004 => Uint8::from_le_stream(bytes).map(Self::BatteryRatedVoltage),
            0x0005 => BatteryAlarmMask::from_le_stream(bytes).map(Self::BatteryAlarmMask),
            0x0006 => Uint8::from_le_stream(bytes).map(Self::BatteryVoltageMinThreshold),
            0x0007 => Uint8::from_le_stream(bytes).map(Self::BatteryVoltageThreshold1),
            0x0008 => Uint8::from_le_stream(bytes).map(Self::BatteryVoltageThreshold2),
            0x0009 => Uint8::from_le_stream(bytes).map(Self::BatteryVoltageThreshold3),
            0x000a => Uint8::from_le_stream(bytes).map(Self::BatteryPercentageMinThreshold),
            0x000b => Uint8::from_le_stream(bytes).map(Self::BatteryPercentageThreshold1),
            0x000c => Uint8::from_le_stream(bytes).map(Self::BatteryPercentageThreshold2),
            0x000d => Uint8::from_le_stream(bytes).map(Self::BatteryPercentageThreshold3),
            0x000e => BatteryAlarmState::from_le_stream(bytes).map(Self::BatteryAlarmState),
            _ => None,
        }
    }
}
