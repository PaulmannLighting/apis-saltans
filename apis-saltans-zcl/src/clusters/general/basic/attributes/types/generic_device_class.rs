use crate::macros::zcl_attribute_newtype;

zcl_attribute_newtype! {
    /// The generic class of a device.
    pub enum GenericDeviceClass: Enum8 {
        /// A lighting device.
        Lighting = 0x00,
    }
}
