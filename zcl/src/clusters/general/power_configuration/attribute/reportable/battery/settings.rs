use le_stream::{FromLeStream, FromLeStreamTagged};
use repr_discriminant::ReprDiscriminant;

use crate::clusters::general::power_configuration::attribute::BatteryAlarmState;

const MASK: u16 = 0x000f;

/// Available battery settings.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u16)]
#[derive(ReprDiscriminant)]
pub enum Settings {
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
            0x000e => Ok(BatteryAlarmState::from_le_stream(bytes).map(Self::AlarmState)),
            _ => Err(tag),
        }
    }
}
