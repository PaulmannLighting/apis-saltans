//! Readable attributes for the Power Configuration cluster.

use le_stream::FromLeStream;
use repr_discriminant::ReprDiscriminant;
use zigbee::types::{Uint8, Uint16};

pub use self::battery::{Battery, Information, Settings};
use super::MainsAlarmMask;

mod battery;

const MASK: u16 = 0xfff0;

/// Readable attributes.
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

impl FromLeStream for Attribute {
    fn from_le_stream<T>(mut bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        let tag = u16::from_le_stream(&mut bytes)?;

        match tag {
            0x0000 => Uint16::from_le_stream(bytes).map(Self::MainsVoltage),
            0x0001 => Uint8::from_le_stream(bytes).map(Self::MainsFrequency),
            0x0010 => MainsAlarmMask::from_le_stream(bytes).map(Self::AlarmMask),
            0x0011 => Uint16::from_le_stream(bytes).map(Self::VoltageMinThreshold),
            0x0012 => Uint16::from_le_stream(bytes).map(Self::VoltageMaxThreshold),
            0x0013 => Uint16::from_le_stream(bytes).map(Self::VoltageDwellTripPoint),
            tag if tag & MASK == 0x0020 || tag & MASK == 0x0030 => {
                Battery::from_le_stream_tagged(tag, bytes).map(Self::Battery)
            }
            tag if tag & MASK == 0x0040 || tag & MASK == 0x0050 => {
                Battery::from_le_stream_tagged(tag, bytes).map(Self::Battery2)
            }
            tag if tag & MASK == 0x0060 || tag & MASK == 0x0070 => {
                Battery::from_le_stream_tagged(tag, bytes).map(Self::Battery3)
            }
            _ => None,
        }
    }
}
