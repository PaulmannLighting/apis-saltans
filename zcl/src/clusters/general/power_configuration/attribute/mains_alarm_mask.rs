use bitflags::bitflags;
use le_stream::derive::{FromLeStream, ToLeStream};

/// Available mains alarms.
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent)
)]
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
#[repr(transparent)]
pub struct MainsAlarmMask(u8);

bitflags! {
    impl MainsAlarmMask: u8 {
        /// Mains voltage is too low.
        const MAINS_VOLTAGE_TOO_LOW = 0b0000_0001;
        /// Mains voltage is too high.
        const MAINS_VOLTAGE_TOO_HIGH = 0b0000_0010;
        /// Mains power supply is lost or unavailable (device may be running on battery).
        const MAINS_POWER_SUPPLY_LOST = 0b0000_0100;
    }
}
