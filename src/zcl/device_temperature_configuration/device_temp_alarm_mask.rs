use bitflags::bitflags;
use le_stream::derive::{FromLeStream, ToLeStream};

/// Device temperature alarm mask.
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
#[repr(transparent)]
pub struct DeviceTempAlarmMask(u8);

bitflags! {
    impl DeviceTempAlarmMask: u8 {
        /// The device temperature is too low.
        const DEVICE_TEMPERATURE_TOO_LOW = 0b1 << 0;
        /// The device temperature is too high.
        const DEVICE_TEMPERATURE_TOO_HIGH = 0b1 << 1;
    }
}
