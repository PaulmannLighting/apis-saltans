use le_stream::FromLeStream;
use repr_discriminant::ReprDiscriminant;

use crate::types::Uint8;

/// Information about the battery status of a device.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u16)]
#[derive(ReprDiscriminant)]
pub enum BatteryInformation {
    /// Battery voltage in 100mV.
    BatteryVoltage(Uint8) = 0x0000,
    /// Battery percentage remaining.
    BatteryPercentageRemaining(Uint8) = 0x0001,
}

impl BatteryInformation {
    pub(crate) fn from_le_stream<T>(mask: u16, bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        match mask {
            0x0000 => Uint8::from_le_stream(bytes).map(Self::BatteryVoltage),
            0x0001 => Uint8::from_le_stream(bytes).map(Self::BatteryPercentageRemaining),
            _ => None,
        }
    }
}
