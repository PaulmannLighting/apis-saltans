//! Attributes of the Alarms cluster.

use apis_saltans_core::ClusterId;

use super::attribute::AlarmCount;
use crate::macros::zcl_attributes;

zcl_attributes! {
    cluster: ClusterId::Alarms;

    /// Number of alarms currently present in the alarm table.
    AlarmCount = 0x0000: AlarmCount { R },
}
