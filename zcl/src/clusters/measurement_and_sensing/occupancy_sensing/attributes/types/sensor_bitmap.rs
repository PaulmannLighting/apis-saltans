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
        } else if value.contains(SensorBitmap::PHYSICAL_CONTACT) {
            Ok(Self::PhysicalContact)
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{SensorBitmap, SensorType};

    #[test]
    fn converts_physical_contact_only_sensor() {
        assert_eq!(
            SensorType::try_from(SensorBitmap::PHYSICAL_CONTACT),
            Ok(SensorType::PhysicalContact)
        );
    }

    #[test]
    fn converts_all_defined_sensor_combinations() {
        let cases = [
            (SensorBitmap::PIR, SensorType::Pir),
            (SensorBitmap::ULTRASONIC, SensorType::Ultrasonic),
            (
                SensorBitmap::PIR | SensorBitmap::ULTRASONIC,
                SensorType::PirAndUltrasonic,
            ),
            (
                SensorBitmap::PIR | SensorBitmap::PHYSICAL_CONTACT,
                SensorType::Pir,
            ),
            (
                SensorBitmap::ULTRASONIC | SensorBitmap::PHYSICAL_CONTACT,
                SensorType::Ultrasonic,
            ),
            (
                SensorBitmap::PIR | SensorBitmap::ULTRASONIC | SensorBitmap::PHYSICAL_CONTACT,
                SensorType::PirAndUltrasonic,
            ),
        ];

        for (bitmap, expected) in cases {
            assert_eq!(SensorType::try_from(bitmap), Ok(expected));
        }
    }
}
