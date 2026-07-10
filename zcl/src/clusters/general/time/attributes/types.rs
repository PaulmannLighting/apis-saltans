//! Attribute value types of the Time cluster.

use zb_core::types::{Int32, UtcTime};

pub use self::time_status::TimeStatus;
use crate::macros::zcl_attribute_newtype;

mod time_status;

zcl_attribute_newtype! {
    /// Current time.
    pub struct Time(UtcTime) => UtcTime;
}

zcl_attribute_newtype! {
    /// Time zone.
    pub struct TimeZone(Int32) => Int32;
}

zcl_attribute_newtype! {
    /// DST start time.
    pub struct DstStart(UtcTime) => UtcTime;
}

zcl_attribute_newtype! {
    /// DST end time.
    pub struct DstEnd(UtcTime) => UtcTime;
}

zcl_attribute_newtype! {
    /// DST time shift.
    pub struct DstShift(Int32) => Int32;
}

zcl_attribute_newtype! {
    /// Standard time.
    pub struct StandardTime(Int32) => Int32;
}

zcl_attribute_newtype! {
    /// Local time.
    pub struct LocalTime(Int32) => Int32;
}

zcl_attribute_newtype! {
    /// Last time the `Time` attribute was set.
    pub struct LastSetTime(UtcTime) => UtcTime;
}

zcl_attribute_newtype! {
    /// Deadline until which the `Time` attribute can be trusted.
    pub struct ValidUntilTime(UtcTime) => UtcTime;
}
