use le_stream::FromLeStream;
use repr_discriminant::repr_discriminant;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr_discriminant(u16, id)]
pub enum BatteryInformation {
    /// Battery voltage in 100mV.
    BatteryVoltage(u8) = 0x0000,
    /// Battery percentage remaining.
    BatteryPercentageRemaining(u8) = 0x0001,
}

impl BatteryInformation {
    pub(crate) fn from_le_stream<T>(mask: u16, bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        match mask {
            0x0000 => u8::from_le_stream(bytes).map(Self::BatteryVoltage),
            0x0001 => u8::from_le_stream(bytes).map(Self::BatteryPercentageRemaining),
            _ => None,
        }
    }
}
