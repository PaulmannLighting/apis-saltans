use super::SensorType;
use crate::macros::zcl_attribute_newtype;

zcl_attribute_newtype! {
    /// Sensor type bitmask.
    pub bitflags SensorBitmap(u8) => Map8 {
        /// PIR sensor.
        const PIR = 0b0000_0001;
        /// Ultrasonic sensor.
        const ULTRASONIC = 0b0000_0010;
        /// Physical contact sensor.
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
