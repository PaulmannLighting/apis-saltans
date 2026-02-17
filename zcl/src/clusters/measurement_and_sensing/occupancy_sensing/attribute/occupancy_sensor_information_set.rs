//! Occupancy information set attributes.

use repr_discriminant::ReprDiscriminant;
use zigbee::Parsable;

pub use self::occupancy::Occupancy;
pub use self::sensor_bitmap::SensorBitmap;
pub use self::sensor_type::SensorType;

mod occupancy;
mod sensor_bitmap;
mod sensor_type;

/// Attributes for the occupancy sensing cluster.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(u16)]
#[derive(ReprDiscriminant)]
pub enum Attribute {
    /// Occupancy status.
    Occupancy(Occupancy) = 0x0000,
    /// Sensor type.
    SensorType(Parsable<u8, SensorType>) = 0x0001,
    /// Sensor type bitmap.
    SensorBitmap(Parsable<u8, SensorBitmap>) = 0x0002,
}
