use bitflags::bitflags;
use le_stream::derive::{FromLeStream, ToLeStream};

/// Options for the On/Off cluster commands.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
#[repr(transparent)]
pub struct Options(u8);

bitflags! {
    impl Options: u8 {
        /// Execute command if, in the On/Off cluster, the OnOff attribute is `0x00` (`FALSE`).
        const ExecuteIfOff = 0b0000_0001;
    }
}
