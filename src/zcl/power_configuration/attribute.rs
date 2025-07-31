use core::iter::Chain;

pub use battery_alarm_mask::BatteryAlarmMask;
pub use battery_alarm_state::BatteryAlarmState;
pub use battery_information::BatteryInformation;
pub use battery_settings::BatterySettings;
pub use battery_size::BatterySize;
use le_stream::{FromLeStream, FromLeStreamTagged, ToLeStream};
pub use mains_alarm_mask::MainsAlarmMask;
use repr_discriminant::ReprDiscriminant;

use crate::types::{Uint8, Uint16};

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
    MainsVoltage(Uint16) = 0x0000,
    /// Mains frequency in Hertz.
    MainsFrequency(Uint8) = 0x0001,
    // Mains settings.
    /// Mains alarms.
    AlarmMask(MainsAlarmMask) = 0x0010,
    /// Mains voltage minimum threshold in 100mV.
    VoltageMinThreshold(Uint16) = 0x0011,
    /// Mains voltage maximum threshold in 100mV.
    VoltageMaxThreshold(Uint16) = 0x0012,
    /// Mains voltage dwell trip point in seconds.
    VoltageDwellTripPoint(Uint16) = 0x0013,
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
            0x0000 => Ok(Uint16::from_le_stream(bytes).map(Self::MainsVoltage)),
            0x0001 => Ok(Uint8::from_le_stream(bytes).map(Self::MainsFrequency)),
            0x0010 => Ok(MainsAlarmMask::from_le_stream(bytes).map(Self::AlarmMask)),
            0x0011 => Ok(Uint16::from_le_stream(bytes).map(Self::VoltageMinThreshold)),
            0x0012 => Ok(Uint16::from_le_stream(bytes).map(Self::VoltageMaxThreshold)),
            0x0013 => Ok(Uint16::from_le_stream(bytes).map(Self::VoltageDwellTripPoint)),
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

    use crate::types::{String, Uint8, Uint16, Uint32};
    use crate::zcl::power_configuration::{
        BatteryAlarmMask, BatteryAlarmState, BatterySize, MainsAlarmMask,
    };

    /// Little endian stream iterator for the [`Attribute`](crate::zcl::power_configuration::Attribute)
    /// in the Power Configuration cluster.
    pub enum Attribute {
        Uint8(<Uint8 as ToLeStream>::Iter),
        Uint16(<Uint16 as ToLeStream>::Iter),
        Uint32(<Uint32 as ToLeStream>::Iter),
        String16(<String<16> as ToLeStream>::Iter),
    }

    impl Iterator for Attribute {
        type Item = u8;

        fn next(&mut self) -> Option<Self::Item> {
            match self {
                Self::Uint8(iter) => iter.next(),
                Self::Uint16(iter) => iter.next(),
                Self::Uint32(iter) => iter.next(),
                Self::String16(iter) => iter.next(),
            }
        }
    }

    impl From<Uint8> for Attribute {
        fn from(value: Uint8) -> Self {
            Self::Uint8(value.to_le_stream())
        }
    }

    impl From<Uint16> for Attribute {
        fn from(value: Uint16) -> Self {
            Self::Uint16(value.to_le_stream())
        }
    }

    impl From<Uint32> for Attribute {
        fn from(value: Uint32) -> Self {
            Self::Uint32(value.to_le_stream())
        }
    }

    impl From<MainsAlarmMask> for Attribute {
        fn from(value: MainsAlarmMask) -> Self {
            Self::Uint8(value.to_le_stream())
        }
    }

    impl From<BatterySize> for Attribute {
        fn from(value: BatterySize) -> Self {
            Self::Uint8(value.to_le_stream())
        }
    }

    impl From<BatteryAlarmMask> for Attribute {
        fn from(value: BatteryAlarmMask) -> Self {
            Self::Uint8(value.to_le_stream())
        }
    }

    impl From<BatteryAlarmState> for Attribute {
        fn from(value: BatteryAlarmState) -> Self {
            Self::Uint32(value.to_le_stream())
        }
    }

    impl From<String<16>> for Attribute {
        fn from(value: String<16>) -> Self {
            Self::String16(value.to_le_stream())
        }
    }
}
