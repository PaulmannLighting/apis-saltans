use bitflags::bitflags;
use le_stream::{FromLeStream, ToLeStream};

use super::SensorType;

/// Sensor type bitmask.
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
pub struct SensorBitmap(u8);

bitflags! {
    impl SensorBitmap: u8 {
        const PIR = 0b0000_0001;
        const ULTRASONIC = 0b0000_0010;
        const PHYSICAL_CONTACT = 0b0000_0100;
    }
}

impl TryFrom<SensorBitmap> for SensorType {
    type Error = ();

    fn try_from(value: SensorBitmap) -> Result<Self, Self::Error> {
        if value
            .contains(SensorBitmap::PIR | SensorBitmap::ULTRASONIC | SensorBitmap::PHYSICAL_CONTACT)
        {
            Ok(Self::PirAndUltrasonic)
        } else if value.contains(SensorBitmap::ULTRASONIC | SensorBitmap::PHYSICAL_CONTACT) {
            Ok(Self::Ultrasonic)
        } else if value.contains(SensorBitmap::PIR | SensorBitmap::PHYSICAL_CONTACT) {
            Ok(Self::Pir)
        } else if value.contains(SensorBitmap::PIR | SensorBitmap::ULTRASONIC) {
            Ok(Self::PirAndUltrasonic)
        } else if value.contains(SensorBitmap::ULTRASONIC) {
            Ok(Self::Ultrasonic)
        } else if value.contains(SensorBitmap::PIR) {
            Ok(Self::Pir)
        } else {
            Err(())
        }
    }
}
