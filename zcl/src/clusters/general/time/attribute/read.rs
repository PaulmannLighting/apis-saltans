//! Readable attributes for the Time cluster.

use le_stream::FromLeStreamTagged;
use repr_discriminant::ReprDiscriminant;
use zigbee::types::UtcTime;

use crate::general::time::attribute::TimeStatus;

/// Readable attributes for the Time cluster.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(u16)]
#[derive(ReprDiscriminant, FromLeStreamTagged)]
pub enum Attribute {
    /// Current time.
    Time(UtcTime) = 0x0000,
    /// Time status.
    TimeStatus(TimeStatus) = 0x0001,
    /// Time zone.
    TimeZone(i32) = 0x0002,
    /// DST start time.
    DstStart(UtcTime) = 0x0003,
    /// DST end time.
    DstEnd(UtcTime) = 0x0004,
    /// DST time shift.
    DstShift(i32) = 0x0005,
    /// Standard time.
    StandardTime(UtcTime) = 0x0006,
    /// Local time.
    LocalTime(UtcTime) = 0x0007,
    /// Last time the `Time` attribute was set.
    LastSetTime(UtcTime) = 0x0008,
    /// Deadline until which the `Time` attribute can be trusted.
    ValidUntilTime(UtcTime) = 0x0009,
}
