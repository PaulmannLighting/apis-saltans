use crate::macros::zcl_attribute_newtype;

zcl_attribute_newtype! {
    /// Available battery sizes.
    pub enum BatterySize: Enum8 {
        /// No battery is present.
        NoBattery = 0x00,
        /// Built-in battery.
        BuiltInBattery = 0x01,
        /// Other battery types.
        Other = 0x02,
        /// AA battery.
        AA = 0x03,
        /// AAA battery.
        AAA = 0x04,
        /// C battery.
        C = 0x05,
        /// D battery.
        D = 0x06,
        /// CR2 battery.
        CR2 = 0x07,
        /// CR123A battery.
        CR123A = 0x08,
        /// Unknown battery size.
        Unknown = 0xff,
    }
}
