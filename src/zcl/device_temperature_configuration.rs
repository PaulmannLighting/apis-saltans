//! Device Temperature Configuration Cluster.

pub use attribute::Attribute;
pub use device_temp_alarm_mask::DeviceTempAlarmMask;
pub use temp_threshold::TempThreshold;
pub use temperature::Temperature;

mod attribute;
mod device_temp_alarm_mask;
mod temp_threshold;
mod temperature;

const CLUSTER_ID: u16 = 0x0002;
