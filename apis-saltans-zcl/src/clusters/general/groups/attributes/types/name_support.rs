use crate::macros::zcl_attribute_newtype;

zcl_attribute_newtype! {
    /// Flag indicating whether the group name is supported by the device.
    pub enum NameSupport: Map8 {
        /// The device does not support group names.
        Unsupported = 0x00,
        /// The device supports group names.
        Supported = 0x01,
    }
}
