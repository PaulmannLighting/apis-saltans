use le_stream::FromLeStream;
use repr_discriminant::repr_discriminant;

/// Mains Information attribute.
#[repr_discriminant(u16)]
pub enum MainsInformation {
    /// Mains voltage in 100mV.
    MainsVoltage(u16) = 0x0000,
    /// Mains frequency in Hertz.
    MainsFrequency(u8) = 0x0001,
}

impl MainsInformation {
    pub(crate) fn from_le_stream<T>(id: u16, mut bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        match id {
            0x0000 => u16::from_le_stream(&mut bytes).map(Self::MainsVoltage),
            0x0001 => u8::from_le_stream(&mut bytes).map(Self::MainsFrequency),
            _ => None,
        }
    }
}
