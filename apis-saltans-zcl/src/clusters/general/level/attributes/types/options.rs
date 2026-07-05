use crate::macros::zcl_attribute_newtype;

zcl_attribute_newtype! {
    /// Options attribute for the Level cluster.
    pub bitflags Options(u8) => Map8 {
        /// Execute the command if the device is off.
        const EXECUTE_IF_OFF = 0b0000_0001;
    }
}
