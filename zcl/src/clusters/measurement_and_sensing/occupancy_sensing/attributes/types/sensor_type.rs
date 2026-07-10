use crate::macros::zcl_attribute_newtype;

zcl_attribute_newtype! {
    /// Occupancy sensor type.
    pub enum SensorType: Enum8 {
        /// PIR sensor.
        Pir = 0x00,
        /// Ultrasonic sensor.
        Ultrasonic = 0x01,
        /// PIR and ultrasonic sensor.
        PirAndUltrasonic = 0x02,
        /// Physical contact sensor.
        PhysicalContact = 0x03,
    }
}
