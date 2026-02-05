use bitflags::bitflags;
use le_stream::{FromLeStream, ToLeStream};

/// Control field for the On with Timed Off command.
#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, FromLeStream, ToLeStream,
)]
pub struct OnOffControl(u8);

bitflags! {
    impl OnOffControl: u8 {
        /// Only accept the command when the device is currently on.
        const ACCEPT_ONLY_WHEN_ON = 0b0000_0001;
    }
}
