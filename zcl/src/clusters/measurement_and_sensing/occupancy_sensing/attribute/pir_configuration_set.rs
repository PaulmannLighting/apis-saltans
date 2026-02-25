//! Configuration attributes of PIR sensors.

use le_stream::FromLeStreamTagged;
use repr_discriminant::ReprDiscriminant;
use zigbee::types::{Uint8, Uint16};

/// Available configuration attributes of PIR sensors.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, FromLeStreamTagged)]
#[repr(u16)]
#[derive(ReprDiscriminant)]
pub enum Attribute {
    /// Delay in seconds before the sensor changes from occupied to unoccupied.
    OccupiedToUnoccupiedDelay(Uint16) = 0x0010,
    /// Delay in seconds before the sensor changes from unoccupied to occupied.
    UnoccupiedToOccupiedDelay(Uint16) = 0x0011,
    /// Number of movement detection events before the sensor changes state.
    UnoccupiedToOccupiedThreshold(Uint8) = 0x0012,
}
