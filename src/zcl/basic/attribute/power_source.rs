use le_stream::{FromLeStream, ToLeStream};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

/// Device power source attribute.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromPrimitive)]
#[repr(u8)]
pub enum PowerSource {
    /// The power source is unknown.
    Unknown = 0x00,
    /// The power source is mains single phase.
    MainsSinglePhase = 0x01,
    /// The power source is mains 3-phase.
    MainsThreePhase = 0x02,
    /// The power source is a battery.
    Battery = 0x03,
    /// The power source is a DC source.
    DcSource = 0x04,
    /// The power source is an emergency mains supply that is constantly powered.
    EmergencyMainsConstantlyPowered = 0x05,
    /// The power source is an emergency mains supply that is powered through a transfer switch.
    EmergencyMainsAndTransferSwitch = 0x06,
}

impl FromLeStream for PowerSource {
    fn from_le_stream<T>(bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        u8::from_le_stream(bytes).and_then(Self::from_u8)
    }
}

impl ToLeStream for PowerSource {
    type Iter = <u8 as ToLeStream>::Iter;

    fn to_le_stream(self) -> Self::Iter {
        (self as u8).to_le_stream()
    }
}

#[cfg(test)]
#[cfg(feature = "alloc")]
mod tests {
    use alloc::vec;
    use alloc::vec::Vec;

    use super::*;

    #[test]
    fn power_source_from_le_stream() {
        let bytes = vec![0x01];
        let power_source = PowerSource::from_le_stream(bytes.into_iter()).unwrap();
        assert_eq!(power_source, PowerSource::MainsSinglePhase);
    }

    #[test]
    fn power_source_to_le_stream() {
        let power_source = PowerSource::Battery;
        let bytes: Vec<u8> = power_source.to_le_stream().collect();
        assert_eq!(bytes, vec![0x03]);
    }
}
