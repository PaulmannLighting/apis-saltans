//! Configuration attributes of ultrasonic sensors.

use le_stream::FromLeStream;
use repr_discriminant::ReprDiscriminant;
use apis_saltans_core::types::{Uint8, Uint16};

/// Available configuration attributes of ultrasonic sensors.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, FromLeStream)]
#[repr(u16)]
#[derive(ReprDiscriminant)]
pub enum Attribute {
    /// Delay in seconds before the sensor changes from occupied to unoccupied.
    OccupiedToUnoccupiedDelay(Uint16) = 0x0020,

    /// Delay in seconds before the sensor changes from unoccupied to occupied.
    UnoccupiedToOccupiedDelay(Uint16) = 0x0021,

    /// Number of movement detection events before the sensor changes state.
    UnoccupiedToOccupiedThreshold(Uint8) = 0x0022,
}
