use bitflags::bitflags;
use le_stream::{FromLeStream, ToLeStream};

/// Options attribute for the Level cluster.
#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, FromLeStream, ToLeStream,
)]
pub struct Options(u8);

bitflags! {
    impl Options: u8 {
        /// Execute the command if the device is off.
        const EXECUTE_IF_OFF = 0b0000_0001;
    }
}
