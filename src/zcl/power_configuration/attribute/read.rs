use battery::Battery;
use le_stream::{FromLeStream, FromLeStreamTagged};
use repr_discriminant::ReprDiscriminant;
use zigbee::types::{Uint16, Uint8};

use super::mains_alarm_mask::MainsAlarmMask;

mod battery;
mod battery_information;
mod battery_settings;

const MASK: u16 = 0xfff0;

/// Power configuration cluster attribute.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[repr(u16)]
#[derive(ReprDiscriminant)]
pub enum Attribute {
    /// Mains voltage in 100mV.
    MainsVoltage(Uint16) = 0x0000,
    /// Mains frequency in Hertz.
    MainsFrequency(Uint8) = 0x0001,
    /// Mains alarms.
    AlarmMask(MainsAlarmMask) = 0x0010,
    /// Mains voltage minimum threshold in 100mV.
    VoltageMinThreshold(Uint16) = 0x0011,
    /// Mains voltage maximum threshold in 100mV.
    VoltageMaxThreshold(Uint16) = 0x0012,
    /// Mains voltage dwell trip point in seconds.
    VoltageDwellTripPoint(Uint16) = 0x0013,
    /// Primary battery data.
    Battery(Battery) = 0x0020,
    /// Secondary battery data.
    Battery2(Battery) = 0x0040,
    /// Tertiary battery data.
    Battery3(Battery) = 0x0060,
}

impl Attribute {
    /// Returns the attribute ID.
    #[must_use]
    pub const fn id(&self) -> u16 {
        match self {
            Self::Battery(battery) | Self::Battery2(battery) | Self::Battery3(battery) => {
                self.discriminant() | battery.mask()
            }
            _ => self.discriminant(),
        }
    }
}

impl FromLeStreamTagged for Attribute {
    type Tag = u16;

    fn from_le_stream_tagged<T>(tag: Self::Tag, bytes: T) -> Result<Option<Self>, Self::Tag>
    where
        T: Iterator<Item = u8>,
    {
        match tag {
            0x0000 => Ok(Uint16::from_le_stream(bytes).map(Self::MainsVoltage)),
            0x0001 => Ok(Uint8::from_le_stream(bytes).map(Self::MainsFrequency)),
            0x0010 => Ok(MainsAlarmMask::from_le_stream(bytes).map(Self::AlarmMask)),
            0x0011 => Ok(Uint16::from_le_stream(bytes).map(Self::VoltageMinThreshold)),
            0x0012 => Ok(Uint16::from_le_stream(bytes).map(Self::VoltageMaxThreshold)),
            0x0013 => Ok(Uint16::from_le_stream(bytes).map(Self::VoltageDwellTripPoint)),
            tag if tag & MASK == 0x0020 || tag & MASK == 0x0030 => {
                Ok(Battery::from_le_stream_tagged(tag, bytes)?.map(Self::Battery))
            }
            tag if tag & MASK == 0x0040 || tag & MASK == 0x0050 => {
                Ok(Battery::from_le_stream_tagged(tag, bytes)?.map(Self::Battery2))
            }
            tag if tag & MASK == 0x0060 || tag & MASK == 0x0070 => {
                Ok(Battery::from_le_stream_tagged(tag, bytes)?.map(Self::Battery3))
            }
            unknown => Err(unknown),
        }
    }
}
