use le_stream::{FromLeStream, FromLeStreamTagged};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use repr_discriminant::ReprDiscriminant;
use zigbee::types::{String, Type, Uint8, Uint16};
use zigbee::{ClusterId, ClusterSpecific};

use crate::clusters::general::power_configuration::attribute::{
    BatteryAlarmMask, BatteryAlarmState, BatterySize,
};
use crate::{InvalidType, ReadableAttribute};

const MASK: u16 = 0x000f;

/// Available battery settings.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u16)]
#[derive(ReprDiscriminant)]
pub enum Settings {
    /// Name of the battery manufacturer.
    Manufacturer(String<16>) = 0x0000,
    /// The battery size.
    Size(BatterySize) = 0x0001,
    /// The battery ampere-hour rating in 10mAHr.
    AHrRating(Uint16) = 0x0002,
    /// Number of battery cells.
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
            0x0001 => Ok(BatterySize::from_le_stream(bytes).map(Self::Size)),
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

impl TryFrom<(Id, Type)> for Settings {
    type Error = InvalidType<Id>;

    #[expect(clippy::too_many_lines)]
    fn try_from((id, typ): (Id, Type)) -> Result<Self, Self::Error> {
        match id {
            Id::Manufacturer => {
                if let Type::String(string) = typ {
                    match string.truncate() {
                        Ok(string) => Ok(Self::Manufacturer(string)),
                        Err(string) => Err(InvalidType::new(id, Type::String(string))),
                    }
                } else {
                    Err(InvalidType::new(id, typ))
                }
            }
            Id::Size => {
                if let Type::Enum8(size) = typ
                    && let Ok(battery_size) = BatterySize::try_from(size)
                {
                    Ok(Self::Size(battery_size))
                } else {
                    Err(InvalidType::new(id, typ))
                }
            }
            Id::AHrRating => {
                if let Type::Uint16(ahr_rating) = typ {
                    Ok(Self::AHrRating(ahr_rating))
                } else {
                    Err(InvalidType::new(id, typ))
                }
            }
            Id::Quantity => {
                if let Type::Uint8(quantity) = typ {
                    Ok(Self::Quantity(quantity))
                } else {
                    Err(InvalidType::new(id, typ))
                }
            }
            Id::RatedVoltage => {
                if let Type::Uint8(rated_voltage) = typ {
                    Ok(Self::RatedVoltage(rated_voltage))
                } else {
                    Err(InvalidType::new(id, typ))
                }
            }
            Id::AlarmMask => {
                if let Type::Enum8(alarm_mask) = typ {
                    Ok(Self::AlarmMask(BatteryAlarmMask::from_bits_retain(
                        alarm_mask.as_u8(),
                    )))
                } else {
                    Err(InvalidType::new(id, typ))
                }
            }
            Id::VoltageMinThreshold => {
                if let Type::Uint8(voltage_min_threshold) = typ {
                    Ok(Self::VoltageMinThreshold(voltage_min_threshold))
                } else {
                    Err(InvalidType::new(id, typ))
                }
            }
            Id::VoltageThreshold1 => {
                if let Type::Uint8(voltage_threshold1) = typ {
                    Ok(Self::VoltageThreshold1(voltage_threshold1))
                } else {
                    Err(InvalidType::new(id, typ))
                }
            }
            Id::VoltageThreshold2 => {
                if let Type::Uint8(voltage_threshold2) = typ {
                    Ok(Self::VoltageThreshold2(voltage_threshold2))
                } else {
                    Err(InvalidType::new(id, typ))
                }
            }
            Id::VoltageThreshold3 => {
                if let Type::Uint8(voltage_threshold3) = typ {
                    Ok(Self::VoltageThreshold3(voltage_threshold3))
                } else {
                    Err(InvalidType::new(id, typ))
                }
            }
            Id::PercentageMinThreshold => {
                if let Type::Uint8(percentage_min_threshold) = typ {
                    Ok(Self::PercentageMinThreshold(percentage_min_threshold))
                } else {
                    Err(InvalidType::new(id, typ))
                }
            }
            Id::PercentageThreshold1 => {
                if let Type::Uint8(percentage_threshold1) = typ {
                    Ok(Self::PercentageThreshold1(percentage_threshold1))
                } else {
                    Err(InvalidType::new(id, typ))
                }
            }
            Id::PercentageThreshold2 => {
                if let Type::Uint8(percentage_threshold2) = typ {
                    Ok(Self::PercentageThreshold2(percentage_threshold2))
                } else {
                    Err(InvalidType::new(id, typ))
                }
            }
            Id::PercentageThreshold3 => {
                if let Type::Uint8(percentage_threshold3) = typ {
                    Ok(Self::PercentageThreshold3(percentage_threshold3))
                } else {
                    Err(InvalidType::new(id, typ))
                }
            }
            Id::AlarmState => {
                if let Type::Uint32(alarm_state) = typ {
                    Ok(Self::AlarmState(BatteryAlarmState::from_bits_retain(
                        alarm_state.as_u32(),
                    )))
                } else {
                    Err(InvalidType::new(id, typ))
                }
            }
        }
    }
}

/// Available battery settings.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromPrimitive)]
#[repr(u16)]
pub enum Id {
    /// Name of the battery manufacturer.
    Manufacturer = 0x0000,
    /// The battery size.
    Size = 0x0001,
    /// The battery ampere-hour rating in 10mAHr.
    AHrRating = 0x0002,
    /// Number of battery cells.
    Quantity = 0x0003,
    /// The battery rated voltage in 100mV.
    RatedVoltage = 0x0004,
    /// The battery alarm mask.
    AlarmMask = 0x0005,
    /// The minimum battery voltage threshold in 100mV.
    VoltageMinThreshold = 0x0006,
    /// The first battery voltage threshold in 100mV.
    VoltageThreshold1 = 0x0007,
    /// The second battery voltage threshold in 100mV.
    VoltageThreshold2 = 0x0008,
    /// The third battery voltage threshold in 100mV.
    VoltageThreshold3 = 0x0009,
    /// The minimum battery percentage threshold.
    PercentageMinThreshold = 0x000a,
    /// The first battery percentage threshold.
    PercentageThreshold1 = 0x000b,
    /// The second battery percentage threshold.
    PercentageThreshold2 = 0x000c,
    /// The third battery percentage threshold.
    PercentageThreshold3 = 0x000d,
    /// The battery alarm state.
    AlarmState = 0x000e,
}

impl ClusterSpecific for Id {
    const CLUSTER: ClusterId = ClusterId::PowerConfiguration;
}

impl ReadableAttribute for Id {
    type Attribute = Settings;
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
