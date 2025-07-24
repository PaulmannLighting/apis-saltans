use core::iter::Chain;

pub use battery_alarm_mask::BatteryAlarmMask;
pub use battery_alarm_state::BatteryAlarmState;
pub use battery_information::BatteryInformation;
pub use battery_settings::BatterySettings;
pub use battery_size::BatterySize;
use le_stream::{FromLeStream, FromLeStreamTagged, ToLeStream};
pub use mains_alarm_mask::MainsAlarmMask;
use repr_discriminant::ReprDiscriminant;

mod battery_alarm_mask;
mod battery_alarm_state;
mod battery_information;
mod battery_settings;
mod battery_size;
mod mains_alarm_mask;

const ATTRIBUTE_MASK: u16 = 0x000f;

/// Power configuration cluster attribute.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u16)]
#[derive(ReprDiscriminant)]
pub enum Attribute {
    // Mains information.
    /// Mains voltage in 100mV.
    MainsVoltage(u16) = 0x0000,
    /// Mains frequency in Hertz.
    MainsFrequency(u8) = 0x0001,
    // Mains settings.
    /// Mains alarms.
    AlarmMask(MainsAlarmMask) = 0x0010,
    /// Mains voltage minimum threshold in 100mV.
    VoltageMinThreshold(u16) = 0x0011,
    /// Mains voltage maximum threshold in 100mV.
    VoltageMaxThreshold(u16) = 0x0012,
    /// Mains voltage dwell trip point in seconds.
    VoltageDwellTripPoint(u16) = 0x0013,
    /// Battery information.
    BatteryInformation(BatteryInformation) = 0x0020,
    /// Battery settings.
    BatterySettings(BatterySettings) = 0x0030,
    /// Battery source 2 information.
    BatterySource2Information(BatteryInformation) = 0x0040,
    /// Battery source 2 settings.
    BatterySource2Settings(BatterySettings) = 0x0050,
    /// Battery source 3 information.
    BatterySource3Information(BatteryInformation) = 0x0060,
    /// Battery source 3 settings.
    BatterySource3Settings(BatterySettings) = 0x0070,
}

impl Attribute {
    /// Returns the attribute ID.
    #[must_use]
    pub const fn id(&self) -> u16 {
        match self {
            Self::BatteryInformation(info)
            | Self::BatterySource2Information(info)
            | Self::BatterySource3Information(info) => self.discriminant() | info.discriminant(),
            Self::BatterySettings(settings)
            | Self::BatterySource2Settings(settings)
            | Self::BatterySource3Settings(settings) => {
                self.discriminant() | settings.discriminant()
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
            0x0000 => Ok(u16::from_le_stream(bytes).map(Self::MainsVoltage)),
            0x0001 => Ok(u8::from_le_stream(bytes).map(Self::MainsFrequency)),
            0x0010 => Ok(MainsAlarmMask::from_le_stream(bytes).map(Self::AlarmMask)),
            0x0011 => Ok(u16::from_le_stream(bytes).map(Self::VoltageMinThreshold)),
            0x0012 => Ok(u16::from_le_stream(bytes).map(Self::VoltageMaxThreshold)),
            0x0013 => Ok(u16::from_le_stream(bytes).map(Self::VoltageDwellTripPoint)),
            id @ 0x0020..=0x002f => Ok(BatteryInformation::from_le_stream(
                id & ATTRIBUTE_MASK,
                bytes,
            )
            .map(Self::BatteryInformation)),
            id @ 0x0030..=0x003f => Ok(BatterySettings::from_le_stream(id & ATTRIBUTE_MASK, bytes)
                .map(Self::BatterySettings)),
            id @ 0x0040..=0x004f => Ok(BatteryInformation::from_le_stream(
                id & ATTRIBUTE_MASK,
                bytes,
            )
            .map(Self::BatterySource2Information)),
            id @ 0x0050..=0x005f => Ok(BatterySettings::from_le_stream(id & ATTRIBUTE_MASK, bytes)
                .map(Self::BatterySource2Settings)),
            id @ 0x0060..=0x006f => Ok(BatteryInformation::from_le_stream(
                id & ATTRIBUTE_MASK,
                bytes,
            )
            .map(Self::BatterySource3Information)),
            id @ 0x0070..=0x007f => Ok(BatterySettings::from_le_stream(id & ATTRIBUTE_MASK, bytes)
                .map(Self::BatterySource3Settings)),
            unknown => Err(unknown),
        }
    }
}

impl ToLeStream for Attribute {
    type Iter = Chain<<u16 as ToLeStream>::Iter, iterator::Attribute>;

    fn to_le_stream(self) -> Self::Iter {
        let id = self.id();
        let payload_iterator: iterator::Attribute = match self {
            Self::MainsVoltage(voltage)
            | Self::VoltageMinThreshold(voltage)
            | Self::VoltageMaxThreshold(voltage) => voltage.into(),
            Self::MainsFrequency(value) => value.into(),
            Self::AlarmMask(mask) => mask.into(),
            Self::VoltageDwellTripPoint(value) => value.into(),
            Self::BatteryInformation(info)
            | Self::BatterySource2Information(info)
            | Self::BatterySource3Information(info) => match info {
                BatteryInformation::BatteryVoltage(voltage) => voltage.into(),
                BatteryInformation::BatteryPercentageRemaining(percentage) => percentage.into(),
            },
            Self::BatterySettings(settings)
            | Self::BatterySource2Settings(settings)
            | Self::BatterySource3Settings(settings) => match settings {
                BatterySettings::BatteryManufacturer(manufacturer) => manufacturer.into(),
                BatterySettings::BatterySize(size) => size.into(),
                BatterySettings::BatteryAHrRating(rating) => rating.into(),
                BatterySettings::BatteryQuantity(quantity) => quantity.into(),
                BatterySettings::BatteryRatedVoltage(voltage) => voltage.into(),
                BatterySettings::BatteryAlarmMask(mask) => mask.into(),
                BatterySettings::BatteryVoltageMinThreshold(threshold)
                | BatterySettings::BatteryVoltageThreshold1(threshold)
                | BatterySettings::BatteryVoltageThreshold2(threshold)
                | BatterySettings::BatteryVoltageThreshold3(threshold)
                | BatterySettings::BatteryPercentageMinThreshold(threshold)
                | BatterySettings::BatteryPercentageThreshold1(threshold)
                | BatterySettings::BatteryPercentageThreshold2(threshold)
                | BatterySettings::BatteryPercentageThreshold3(threshold) => threshold.into(),
                BatterySettings::BatteryAlarmState(state) => state.into(),
            },
        };
        id.to_le_stream().chain(payload_iterator)
    }
}

/// Iterator for `Attribute` payloads.
mod iterator {
    use le_stream::ToLeStream;

    use crate::types::String16;
    use crate::zcl::power_configuration::{
        BatteryAlarmMask, BatteryAlarmState, BatterySize, MainsAlarmMask,
    };

    /// Little endian stream iterator for the [`Attribute`](crate::zcl::power_configuration::Attribute)
    /// in the Power Configuration cluster.
    pub enum Attribute {
        U8(<u8 as ToLeStream>::Iter),
        U16(<u16 as ToLeStream>::Iter),
        U32(<u32 as ToLeStream>::Iter),
        String16(<String16 as ToLeStream>::Iter),
        MainsAlarmMask(<MainsAlarmMask as ToLeStream>::Iter),
        BatterySize(<BatterySize as ToLeStream>::Iter),
        BatteryAlarmMask(<BatteryAlarmMask as ToLeStream>::Iter),
        BatteryAlarmState(<BatteryAlarmState as ToLeStream>::Iter),
    }

    impl Iterator for Attribute {
        type Item = u8;

        fn next(&mut self) -> Option<Self::Item> {
            match self {
                Self::U8(iter)
                | Self::MainsAlarmMask(iter)
                | Self::BatterySize(iter)
                | Self::BatteryAlarmMask(iter) => iter.next(),
                Self::U16(iter) => iter.next(),
                Self::U32(iter) | Self::BatteryAlarmState(iter) => iter.next(),
                Self::String16(iter) => iter.next(),
            }
        }
    }

    impl From<u8> for Attribute {
        fn from(value: u8) -> Self {
            Self::U8(value.to_le_stream())
        }
    }

    impl From<u16> for Attribute {
        fn from(value: u16) -> Self {
            Self::U16(value.to_le_stream())
        }
    }

    impl From<u32> for Attribute {
        fn from(value: u32) -> Self {
            Self::U32(value.to_le_stream())
        }
    }

    impl From<String16> for Attribute {
        fn from(value: String16) -> Self {
            Self::String16(value.to_le_stream())
        }
    }

    impl From<MainsAlarmMask> for Attribute {
        fn from(value: MainsAlarmMask) -> Self {
            Self::MainsAlarmMask(value.to_le_stream())
        }
    }

    impl From<BatterySize> for Attribute {
        fn from(value: BatterySize) -> Self {
            Self::BatterySize(value.to_le_stream())
        }
    }

    impl From<BatteryAlarmMask> for Attribute {
        fn from(value: BatteryAlarmMask) -> Self {
            Self::BatteryAlarmMask(value.to_le_stream())
        }
    }

    impl From<BatteryAlarmState> for Attribute {
        fn from(value: BatteryAlarmState) -> Self {
            Self::BatteryAlarmState(value.to_le_stream())
        }
    }
}
