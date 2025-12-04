//! Zigbee Cluster Library (ZCL) Basic Cluster.

pub use self::attribute::{
    AlarmMask, CustomString, DateCode, DeviceEnabled, DisableLocalConfig, GenericDeviceClass,
    PhysicalEnvironment, PowerSource, read, write,
};
pub use self::commands::ResetToFactoryDefaults;

mod attribute;
mod commands;

const CLUSTER_ID: u16 = 0x0000;

/// Basic Cluster commands.
#[derive(Debug)]
pub enum Command {
    /// Reset a device to factory defaults.
    ResetToFactoryDefaults(ResetToFactoryDefaults),
}

/// Basic Cluster responses.
#[derive(Debug)]
pub enum Response {}
