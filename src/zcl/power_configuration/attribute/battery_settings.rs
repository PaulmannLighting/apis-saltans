use le_stream::FromLeStream;
use repr_discriminant::repr_discriminant;

use crate::types::String16;
use crate::zcl::power_configuration::attribute::{
    BatteryAlarmMask, BatteryAlarmState, BatterySize,
};

/// Available battery settings.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr_discriminant(u16, id)]
pub enum BatterySettings {
    /// Name of the battery manufacturer.
    BatteryManufacturer(String16) = 0x0000,
    /// The battery size.
    BatterySize(BatterySize) = 0x0001,
    /// The battery ampere-hour rating in 10mAHr.
    BatteryAHrRating(u16) = 0x0002,
    /// Amount of battery cells.
    BatteryQuantity(u8) = 0x0003,
    /// The battery rated voltage in 100mV.
    BatteryRatedVoltage(u8) = 0x0004,
    /// The battery alarm mask.
    BatteryAlarmMask(BatteryAlarmMask) = 0x0005,
    /// The minimum battery voltage threshold in 100mV.
    BatteryVoltageMinThreshold(u8) = 0x0006,
    /// The first battery voltage threshold in 100mV.
    BatteryVoltageThreshold1(u8) = 0x0007,
    /// The second battery voltage threshold in 100mV.
    BatteryVoltageThreshold2(u8) = 0x0008,
    /// The third battery voltage threshold in 100mV.
    BatteryVoltageThreshold3(u8) = 0x0009,
    /// The minimum battery percentage threshold.
    BatteryPercentageMinThreshold(u8) = 0x000a,
    /// The first battery percentage threshold.
    BatteryPercentageThreshold1(u8) = 0x000b,
    /// The second battery percentage threshold.
    BatteryPercentageThreshold2(u8) = 0x000c,
    /// The third battery percentage threshold.
    BatteryPercentageThreshold3(u8) = 0x000d,
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
            0x0002 => u16::from_le_stream(bytes).map(Self::BatteryAHrRating),
            0x0003 => u8::from_le_stream(bytes).map(Self::BatteryQuantity),
            0x0004 => u8::from_le_stream(bytes).map(Self::BatteryRatedVoltage),
            0x0005 => BatteryAlarmMask::from_le_stream(bytes).map(Self::BatteryAlarmMask),
            0x0006 => u8::from_le_stream(bytes).map(Self::BatteryVoltageMinThreshold),
            0x0007 => u8::from_le_stream(bytes).map(Self::BatteryVoltageThreshold1),
            0x0008 => u8::from_le_stream(bytes).map(Self::BatteryVoltageThreshold2),
            0x0009 => u8::from_le_stream(bytes).map(Self::BatteryVoltageThreshold3),
            0x000a => u8::from_le_stream(bytes).map(Self::BatteryPercentageMinThreshold),
            0x000b => u8::from_le_stream(bytes).map(Self::BatteryPercentageThreshold1),
            0x000c => u8::from_le_stream(bytes).map(Self::BatteryPercentageThreshold2),
            0x000d => u8::from_le_stream(bytes).map(Self::BatteryPercentageThreshold3),
            0x000e => BatteryAlarmState::from_le_stream(bytes).map(Self::BatteryAlarmState),
            _ => None,
        }
    }
}
