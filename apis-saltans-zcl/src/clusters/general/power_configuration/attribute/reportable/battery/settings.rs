use le_stream::FromLeStream;
use repr_discriminant::ReprDiscriminant;

use crate::clusters::general::power_configuration::attribute::BatteryAlarmState;

const MASK: u16 = 0x000f;

/// Available battery settings.
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    expect(clippy::unsafe_derive_deserialize)
)]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u16)]
#[derive(ReprDiscriminant)]
pub enum Settings {
    /// The battery alarm state.
    AlarmState(BatteryAlarmState) = 0x000e,
}

impl Settings {
    pub(crate) fn from_le_stream_tagged<T>(tag: u16, bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        match tag & MASK {
            0x000e => BatteryAlarmState::from_le_stream(bytes).map(Self::AlarmState),
            _ => None,
        }
    }
}
