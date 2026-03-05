use le_stream::{FromLeStream, FromLeStreamTagged};
use repr_discriminant::ReprDiscriminant;
use zigbee::Parsable;
use zigbee::types::{String, Uint8, Uint16};

use crate::clusters::general::power_configuration::attribute::{
    BatteryAlarmMask, BatteryAlarmState, BatterySize,
};

const MASK: u16 = 0x000f;

/// Available battery settings.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u16)]
#[derive(ReprDiscriminant)]
#[expect(variant_size_differences)]
pub enum Settings {
    /// Name of the battery manufacturer.
    Manufacturer(String<16>) = 0x0000,
    /// The battery size.
    Size(Parsable<u8, BatterySize>) = 0x0001,
    /// The battery ampere-hour rating in 10mAHr.
    AHrRating(Uint16) = 0x0002,
    /// Amount of battery cells.
    Quantity(Uint8) = 0x0003,
    /// The battery rated voltage in 100mV.
    RatedVoltage(Uint8) = 0x0004,
    /// The battery alarm mask.
    AlarmMask(BatteryAlarmMask) = 0x0005,
    /// The minimum battery voltage threshold in 100mV.
    VoltageMinThreshold(Uint8) = 0x0006,
    /// The first battery voltage threshold in 100mV.
    VoltageThreshold1(Uint8) = 0x0007,
    /// The second battery voltage threshold in 100mV.
    VoltageThreshold2(Uint8) = 0x0008,
    /// The third battery voltage threshold in 100mV.
    VoltageThreshold3(Uint8) = 0x0009,
    /// The minimum battery percentage threshold.
    PercentageMinThreshold(Uint8) = 0x000a,
    /// The first battery percentage threshold.
    PercentageThreshold1(Uint8) = 0x000b,
    /// The second battery percentage threshold.
    PercentageThreshold2(Uint8) = 0x000c,
    /// The third battery percentage threshold.
    PercentageThreshold3(Uint8) = 0x000d,
    /// The battery alarm state.
    AlarmState(BatteryAlarmState) = 0x000e,
}

impl FromLeStreamTagged for Settings {
    type Tag = u16;

    fn from_le_stream_tagged<T>(tag: Self::Tag, bytes: T) -> Result<Option<Self>, Self::Tag>
    where
        T: Iterator<Item = u8>,
    {
        match tag & MASK {
            0x0000 => Ok(String::<16>::from_le_stream(bytes).map(Self::Manufacturer)),
            0x0001 => Ok(Parsable::from_le_stream(bytes).map(Self::Size)),
            0x0002 => Ok(Uint16::from_le_stream(bytes).map(Self::AHrRating)),
            0x0003 => Ok(Uint8::from_le_stream(bytes).map(Self::Quantity)),
            0x0004 => Ok(Uint8::from_le_stream(bytes).map(Self::RatedVoltage)),
            0x0005 => Ok(BatteryAlarmMask::from_le_stream(bytes).map(Self::AlarmMask)),
            0x0006 => Ok(Uint8::from_le_stream(bytes).map(Self::VoltageMinThreshold)),
            0x0007 => Ok(Uint8::from_le_stream(bytes).map(Self::VoltageThreshold1)),
            0x0008 => Ok(Uint8::from_le_stream(bytes).map(Self::VoltageThreshold2)),
            0x0009 => Ok(Uint8::from_le_stream(bytes).map(Self::VoltageThreshold3)),
            0x000a => Ok(Uint8::from_le_stream(bytes).map(Self::PercentageMinThreshold)),
            0x000b => Ok(Uint8::from_le_stream(bytes).map(Self::PercentageThreshold1)),
            0x000c => Ok(Uint8::from_le_stream(bytes).map(Self::PercentageThreshold2)),
            0x000d => Ok(Uint8::from_le_stream(bytes).map(Self::PercentageThreshold3)),
            0x000e => Ok(BatteryAlarmState::from_le_stream(bytes).map(Self::AlarmState)),
            _ => Err(tag),
        }
    }
}
