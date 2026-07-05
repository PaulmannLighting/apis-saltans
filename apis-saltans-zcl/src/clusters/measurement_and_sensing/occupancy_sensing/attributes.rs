//! Attributes of the Occupancy Sensing cluster.

use apis_saltans_core::ClusterId;
use apis_saltans_core::types::{Type, Uint8, Uint16};

use crate::macros::zcl_attributes;

zcl_attributes! {
    cluster: ClusterId::OccupancySensing;

    /// Occupancy status.
    Occupancy = 0x0000: Type { R, P },
    /// Sensor type.
    SensorType = 0x0001: Type { R },
    /// Sensor type bitmap.
    SensorBitmap = 0x0002: Type { R },
    /// PIR delay in seconds before the sensor changes from occupied to unoccupied.
    PirOccupiedToUnoccupiedDelay = 0x0010: Uint16 { R, W },
    /// PIR delay in seconds before the sensor changes from unoccupied to occupied.
    PirUnoccupiedToOccupiedDelay = 0x0011: Uint16 { R, W },
    /// PIR number of movement detection events before the sensor changes state.
    PirUnoccupiedToOccupiedThreshold = 0x0012: Uint8 { R, W },
    /// Ultrasonic delay in seconds before the sensor changes from occupied to unoccupied.
    UltrasonicOccupiedToUnoccupiedDelay = 0x0020: Uint16 { R, W },
    /// Ultrasonic delay in seconds before the sensor changes from unoccupied to occupied.
    UltrasonicUnoccupiedToOccupiedDelay = 0x0021: Uint16 { R, W },
    /// Ultrasonic number of movement detection events before the sensor changes state.
    UltrasonicUnoccupiedToOccupiedThreshold = 0x0022: Uint8 { R, W },
    /// Physical contact delay in seconds before the sensor changes from occupied to unoccupied.
    PhysicalContactOccupiedToUnoccupiedDelay = 0x0030: Uint16 { R, W },
    /// Physical contact delay in seconds before the sensor changes from unoccupied to occupied.
    PhysicalContactUnoccupiedToOccupiedDelay = 0x0031: Uint16 { R, W },
    /// Physical contact number of movement detection events before the sensor changes state.
    PhysicalContactUnoccupiedToOccupiedThreshold = 0x0032: Uint8 { R, W },
}
