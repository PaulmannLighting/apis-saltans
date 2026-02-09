//! Writable attributes for the Time cluster.

use le_stream::FromLeStreamTagged;
use repr_discriminant::ReprDiscriminant;
use zigbee::types::UtcTime;

use crate::general::time::attribute::TimeStatus;

/// Writable attributes for the Time cluster.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(u16)]
#[derive(ReprDiscriminant, FromLeStreamTagged)]
pub enum Attribute {
    Time(UtcTime) = 0x0000,
    TimeStatus(TimeStatus) = 0x0001,
    TimeZone(i32) = 0x0002,
    DstStart(UtcTime) = 0x0003,
    DstEnd(UtcTime) = 0x0004,
    DstShift(i32) = 0x0005,
    ValidUntilTime(UtcTime) = 0x0009,
}
