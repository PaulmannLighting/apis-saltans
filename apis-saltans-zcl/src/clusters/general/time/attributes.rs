//! Attributes of the Time cluster.

use apis_saltans_core::ClusterId;
use apis_saltans_core::types::Type;

use crate::macros::zcl_attributes;

zcl_attributes! {
    cluster: ClusterId::Time;

    /// Current time.
    Time = 0x0000: Type { R, W },
    /// Time status.
    TimeStatus = 0x0001: Type { R, W },
    /// Time zone.
    TimeZone = 0x0002: Type { R, W },
    /// DST start time.
    DstStart = 0x0003: Type { R, W },
    /// DST end time.
    DstEnd = 0x0004: Type { R, W },
    /// DST time shift.
    DstShift = 0x0005: Type { R, W },
    /// Standard time.
    StandardTime = 0x0006: Type { R },
    /// Local time.
    LocalTime = 0x0007: Type { R },
    /// Last time the `Time` attribute was set.
    LastSetTime = 0x0008: Type { R },
    /// Deadline until which the `Time` attribute can be trusted.
    ValidUntilTime = 0x0009: Type { R, W },
}
