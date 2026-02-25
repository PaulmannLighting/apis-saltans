//! Writable attributes.

use repr_discriminant::ReprDiscriminant;
use zigbee::types::Uint16;

/// Attributes for the illuminance level sensing cluster.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(u16)]
#[derive(ReprDiscriminant)]
pub enum Attribute {
    /// Target illuminance level in lux.
    IlluminanceTargetLevel(Uint16) = 0x0010,
}
