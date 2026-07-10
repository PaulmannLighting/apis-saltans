use crate::macros::zcl_attribute_newtype;

zcl_attribute_newtype! {
    /// Device temperature alarm mask.
    pub bitflags AlarmMask(u8) => Map8 {
        /// Device Temperature too low.
        const TOO_LOW = 0b0000_0001;

        /// Device Temperature too high.
        const TOO_HIGH = 0b0000_0010;
    }
}
