//! Illuminance Level Sensing cluster.

pub use attribute::{LevelStatus, LightSensorType};

pub mod attribute;

const CLUSTER_ID: u16 = 0x0401;
