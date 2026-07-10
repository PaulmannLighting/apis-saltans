use crate::macros::zcl_attribute_newtype;

zcl_attribute_newtype! {
    /// Attribute to define the behavior of the On/Off cluster at startup.
    pub enum StartUpOnOff: Enum8 {
        /// Set the `OnOff` attribute to 0 (off).
        Off = 0x00,
        /// Set the `OnOff` attribute to 1 (on).
        On = 0x01,
        /// Toggle the previous value of the `OnOff` attribute.
        Toggle = 0x02,
        /// Set the `OnOff` attribute to its previous value.
        Previous = 0xff,
    }
}
