//! Device Temperature Configuration Cluster.

pub use attribute::Attribute;
pub use device_temp_alarm_mask::DeviceTempAlarmMask;

mod attribute;
mod device_temp_alarm_mask;

const CLUSTER_ID: u16 = 0x0002;
