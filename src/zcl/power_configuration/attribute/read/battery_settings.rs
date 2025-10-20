use le_stream::{FromLeStream, FromLeStreamTagged};
use repr_discriminant::ReprDiscriminant;

use crate::types::{OctStr, String, Uint8, Uint16};
use crate::util::Parsable;
use crate::zcl::power_configuration::attribute::{
    BatteryAlarmMask, BatteryAlarmState, BatterySize,
};

const MASK: u16 = 0x000f;

/// Available battery settings.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u16)]
#[derive(ReprDiscriminant)]
pub enum BatterySettings {
    /// Name of the battery manufacturer.
    BatteryManufacturer(Parsable<OctStr<16>, String<16>>) = 0x0000,
    /// The battery size.
    BatterySize(Parsable<u8, BatterySize>) = 0x0001,
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

impl FromLeStreamTagged for BatterySettings {
    type Tag = u16;

    fn from_le_stream_tagged<T>(tag: Self::Tag, bytes: T) -> Result<Option<Self>, Self::Tag>
    where
        T: Iterator<Item = u8>,
    {
        match tag & MASK {
            0x0000 => Ok(Parsable::from_le_stream(bytes).map(Self::BatteryManufacturer)),
            0x0001 => Ok(Parsable::from_le_stream(bytes).map(Self::BatterySize)),
            0x0002 => Ok(Uint16::from_le_stream(bytes).map(Self::BatteryAHrRating)),
            0x0003 => Ok(Uint8::from_le_stream(bytes).map(Self::BatteryQuantity)),
            0x0004 => Ok(Uint8::from_le_stream(bytes).map(Self::BatteryRatedVoltage)),
            0x0005 => Ok(BatteryAlarmMask::from_le_stream(bytes).map(Self::BatteryAlarmMask)),
            0x0006 => Ok(Uint8::from_le_stream(bytes).map(Self::BatteryVoltageMinThreshold)),
            0x0007 => Ok(Uint8::from_le_stream(bytes).map(Self::BatteryVoltageThreshold1)),
            0x0008 => Ok(Uint8::from_le_stream(bytes).map(Self::BatteryVoltageThreshold2)),
            0x0009 => Ok(Uint8::from_le_stream(bytes).map(Self::BatteryVoltageThreshold3)),
            0x000a => Ok(Uint8::from_le_stream(bytes).map(Self::BatteryPercentageMinThreshold)),
            0x000b => Ok(Uint8::from_le_stream(bytes).map(Self::BatteryPercentageThreshold1)),
            0x000c => Ok(Uint8::from_le_stream(bytes).map(Self::BatteryPercentageThreshold2)),
            0x000d => Ok(Uint8::from_le_stream(bytes).map(Self::BatteryPercentageThreshold3)),
            0x000e => Ok(BatteryAlarmState::from_le_stream(bytes).map(Self::BatteryAlarmState)),
            _ => Err(tag),
        }
    }
}
