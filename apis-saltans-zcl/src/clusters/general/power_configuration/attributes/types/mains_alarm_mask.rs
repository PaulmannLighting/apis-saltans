use crate::macros::zcl_attribute_newtype;

zcl_attribute_newtype! {
    /// Available mains alarms.
    pub bitflags MainsAlarmMask(u8) => Map8 {
        /// Mains voltage is too low.
        const MAINS_VOLTAGE_TOO_LOW = 0b0000_0001;
        /// Mains voltage is too high.
        const MAINS_VOLTAGE_TOO_HIGH = 0b0000_0010;
        /// Mains power supply is lost or unavailable (device may be running on battery).
        const MAINS_POWER_SUPPLY_LOST = 0b0000_0100;
    }
}
