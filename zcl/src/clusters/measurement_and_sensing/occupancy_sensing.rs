//! Occupancy sensing cluster.

pub use self::attributes::{
    Id, Occupancy, Readable, Reportable, SendReport, SensorBitmap, SensorType, Writable,
};

mod attributes;
