use bitflags::bitflags;
use le_stream::derive::{FromLeStream, ToLeStream};

/// Flags for local device configuration functions to be disabled.
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
#[repr(transparent)]
pub struct DisableLocalConfig(u8);

bitflags! {
    impl DisableLocalConfig: u8 {
        /// Reset to factory defaults is enabled.
        const RESET = 0b0000_0001;
        /// Device configuration is enabled.
        const DEVICE_CONFIGURATION = 0b0000_0010;
    }
}
