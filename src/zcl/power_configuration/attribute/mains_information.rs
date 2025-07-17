use repr_discriminant::repr_discriminant;

/// Mains Information attribute.
#[repr_discriminant(u16)]
pub enum MainsInformation {
    /// Mains voltage in 100mV.
    MainsVoltage(u16) = 0x0000,
    /// Mains frequency in Hertz.
    MainsFrequency(u8) = 0x0001,
}
