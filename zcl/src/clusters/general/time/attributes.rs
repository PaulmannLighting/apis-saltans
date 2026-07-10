//! Attributes of the Time cluster.

use zb_core::Cluster;

pub use self::types::{
    DstEnd, DstShift, DstStart, LastSetTime, LocalTime, StandardTime, Time, TimeStatus, TimeZone,
    ValidUntilTime,
};
use crate::macros::zcl_attributes;

mod types;

zcl_attributes! {
    cluster: Cluster::Time;

    /// Current time.
    Time = 0x0000: Time { R, W },
    /// Time status.
    TimeStatus = 0x0001: TimeStatus { R, W },
    /// Time zone.
    TimeZone = 0x0002: TimeZone { R, W },
    /// DST start time.
    DstStart = 0x0003: DstStart { R, W },
    /// DST end time.
    DstEnd = 0x0004: DstEnd { R, W },
    /// DST time shift.
    DstShift = 0x0005: DstShift { R, W },
    /// Standard time.
    StandardTime = 0x0006: StandardTime { R },
    /// Local time.
    LocalTime = 0x0007: LocalTime { R },
    /// Last time the `Time` attribute was set.
    LastSetTime = 0x0008: LastSetTime { R },
    /// Deadline until which the `Time` attribute can be trusted.
    ValidUntilTime = 0x0009: ValidUntilTime { R, W },
}
