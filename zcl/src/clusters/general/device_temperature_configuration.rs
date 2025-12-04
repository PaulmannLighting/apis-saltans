//! Device Temperature Configuration Cluster.

pub use self::device_temp_alarm_mask::DeviceTempAlarmMask;
pub use self::temperature::Temperature;

pub mod attribute;
mod device_temp_alarm_mask;
mod temperature;

const CLUSTER_ID: u16 = 0x0002;

/// Device Temperature Configuration Cluster commands.
#[derive(Debug)]
pub enum Command {}

/// Device Temperature Configuration Cluster responses.
#[derive(Debug)]
pub enum Response {}
