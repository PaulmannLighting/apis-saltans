//! Device Temperature Configuration Cluster.

pub use self::attribute::{readable, writable};
pub use self::device_temp_alarm_mask::DeviceTempAlarmMask;
pub use self::temperature::Temperature;

mod attribute;
mod device_temp_alarm_mask;
mod temperature;

const CLUSTER_ID: u16 = 0x0002;
