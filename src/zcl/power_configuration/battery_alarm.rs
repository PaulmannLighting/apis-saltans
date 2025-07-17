use le_stream::{FromLeStream, ToLeStream};

/// Alarm codes for batteries.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum BatteryAlarm {
    /// `BatteryVoltageMinThreshold` or `BatteryPercentageMinThreshold` reached for Battery Source 1
    Batt1MinThreshold = 0x10,
    /// `BatteryVoltageThreshold1` or `BatteryPercentageThreshold1` reached for Battery Source 1
    Batt1Threshold1 = 0x11,
    /// `BatteryVoltageThreshold2` or `BatteryPercentageThreshold2` reached for Battery Source 1
    Batt1Threshold2 = 0x12,
    /// `BatteryVoltageThreshold3` or `BatteryPercentageThreshold3` reached for Battery Source 1
    Batt1Threshold3 = 0x13,
    /// `BatteryVoltageMinThreshold` or `BatteryPercentageMinThreshold` reached for Battery Source 2
    Batt2MinThreshold = 0x20,
    /// `BatteryVoltageThreshold1` or `BatteryPercentageThreshold1` reached for Battery Source 2
    Batt2Threshold1 = 0x21,
    /// `BatteryVoltageThreshold2` or `BatteryPercentageThreshold2` reached Battery Source 2
    Batt2Threshold2 = 0x22,
    /// `BatteryVoltageThreshold3` or `BatteryPercentageThreshold3` reached Battery Source 2
    Batt2Threshold3 = 0x23,
    /// `BatteryVoltageMinThreshold` or `BatteryPercentageMinThreshold` reached for Battery Source 3
    Batt3MinThreshold = 0x30,
    /// `BatteryVoltageThreshold1` or `BatteryPercentageThreshold1` reached for Battery Source 3
    Batt3Threshold1 = 0x31,
    /// `BatteryVoltageThreshold2` or `BatteryPercentageThreshold2` reached Battery Source 3
    Batt3Threshold2 = 0x32,
    /// `BatteryVoltageThreshold3` or `BatteryPercentageThreshold3` reached Battery Source 3
    Batt3Threshold3 = 0x33,
    /// Mains power supply lost/unavailable (i.e., device is running on battery).
    MainsPowerLost = 0x3a,
    /// Alarm shall not be generated.
    None = 0xff,
}

impl FromLeStream for BatteryAlarm {
    fn from_le_stream<T>(bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        match u8::from_le_stream(bytes)? {
            0x10 => Some(Self::Batt1MinThreshold),
            0x11 => Some(Self::Batt1Threshold1),
            0x12 => Some(Self::Batt1Threshold2),
            0x13 => Some(Self::Batt1Threshold3),
            0x20 => Some(Self::Batt2MinThreshold),
            0x21 => Some(Self::Batt2Threshold1),
            0x22 => Some(Self::Batt2Threshold2),
            0x23 => Some(Self::Batt2Threshold3),
            0x30 => Some(Self::Batt3MinThreshold),
            0x31 => Some(Self::Batt3Threshold1),
            0x32 => Some(Self::Batt3Threshold2),
            0x33 => Some(Self::Batt3Threshold3),
            0x3a => Some(Self::MainsPowerLost),
            0xff => Some(Self::None),
            _ => None,
        }
    }
}

impl ToLeStream for BatteryAlarm {
    type Iter = <u8 as ToLeStream>::Iter;

    fn to_le_stream(self) -> Self::Iter {
        (self as u8).to_le_stream()
    }
}
