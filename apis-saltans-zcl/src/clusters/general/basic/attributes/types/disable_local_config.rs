use crate::macros::zcl_attribute_newtype;

zcl_attribute_newtype! {
    /// Flags for local device configuration functions to be disabled.
    pub bitflags DisableLocalConfig(u8) => Map8 {
        /// Reset to factory defaults is enabled.
        const RESET = 0b0000_0001;
        /// Device configuration is enabled.
        const DEVICE_CONFIGURATION = 0b0000_0010;
    }
}
