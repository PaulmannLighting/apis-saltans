use bitflags::bitflags;
use le_stream::{FromLeStream, ToLeStream};

/// Time status attribute for the Time cluster.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, FromLeStream, ToLeStream)]
pub struct TimeStatus(u8);

bitflags! {
    impl TimeStatus: u8 {
        /// Indicates whether this is a master clock.
        const MASTER = 0b0000_0001;
        /// Indicates whether the time is synchronized.
        const SYNCHRONIZED = 0b0000_0010;
        /// Indicates whether this is a master clock for time zone and DST.
        const MASTER_ZONE_DST = 0b0000_0100;
        /// Indicates whether time synchronization should be superseded.
        const SUPERSEDING = 0b0000_1000;
    }
}
