use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;

/// Occupancy sensor type.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, FromPrimitive, ToPrimitive)]
#[repr(u8)]
pub enum SensorType {
    /// PIR sensor.
    Pir = 0x00,
    /// Ultrasonic sensor.
    Ultrasonic = 0x01,
    /// PIR and ultrasonic sensor.
    PirAndUltrasonic = 0x02,
    /// Physical contact sensor.
    PhysicalContact = 0x03,
}

impl TryFrom<u8> for SensorType {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or(value)
    }
}

impl From<SensorType> for u8 {
    fn from(sensor_type: SensorType) -> Self {
        sensor_type as Self
    }
}
