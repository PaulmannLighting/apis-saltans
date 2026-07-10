use crate::macros::zcl_attribute_newtype;

zcl_attribute_newtype! {
    /// Alarm mask.
    pub bitflags AlarmMask(u8) => Map8 {
        /// General hardware fault.
        const GENERAL_HARDWARE_FAULT = 0b0000_0001;
        /// General software fault.
        const GENERAL_SOFTWARE_FAULT = 0b0000_0010;
    }
}

impl AlarmMask {
    /// Create a new `AlarmMask`.
    #[must_use]
    pub const fn new(mask: u8) -> Self {
        Self(mask)
    }

    /// Return whether this is a global hardware fault.
    #[must_use]
    pub const fn is_general_hardware_fault(self) -> bool {
        self.contains(Self::GENERAL_HARDWARE_FAULT)
    }

    /// Return whether this is a global software fault.
    #[must_use]
    pub const fn is_general_software_fault(self) -> bool {
        self.contains(Self::GENERAL_SOFTWARE_FAULT)
    }
}
