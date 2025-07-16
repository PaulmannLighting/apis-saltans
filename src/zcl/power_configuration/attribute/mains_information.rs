use std::ptr::from_ref;

/// Mains Information attribute.
#[repr(u16)]
pub enum MainsInformation {
    /// Mains voltage in 100mV.
    MainsVoltage(u16) = 0x0000,
    /// Mains frequency in Hertz.
    MainsFrequency(u8) = 0x0001,
}

impl MainsInformation {
    /// Return the enum ID.
    #[must_use]
    pub const fn id(&self) -> u16 {
        // SAFETY: This is safe because `MainsInformation` is repr(u16).
        #[allow(unsafe_code)]
        unsafe {
            *from_ref::<Self>(self).cast::<u16>()
        }
    }
}
