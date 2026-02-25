//! Illuminance Measurement Cluster.

pub use light_sensor_type::{LightSensorType, ManufacturerSpecific};

pub mod attribute;
mod light_sensor_type;

const CLUSTER_ID: u16 = 0x0400;
