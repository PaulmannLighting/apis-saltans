use le_stream::{FromLeStream, ToLeStream};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

/// Device Enabled Attribute.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromPrimitive)]
#[repr(u8)]
pub enum DeviceEnabled {
    /// Device is disabled.
    Disabled = 0x00,
    /// Device is enabled.
    Enabled = 0x01,
}

impl FromLeStream for DeviceEnabled {
    fn from_le_stream<T>(bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        u8::from_le_stream(bytes).and_then(Self::from_u8)
    }
}

impl ToLeStream for DeviceEnabled {
    type Iter = <u8 as ToLeStream>::Iter;

    fn to_le_stream(self) -> Self::Iter {
        (self as u8).to_le_stream()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enabled_from_le_stream() {
        let bytes = vec![0x01];
        let device_enabled = DeviceEnabled::from_le_stream(bytes.into_iter()).unwrap();
        assert_eq!(device_enabled, DeviceEnabled::Enabled);
    }

    #[test]
    fn test_disabled_from_le_stream() {
        let bytes = vec![0x00];
        let device_enabled = DeviceEnabled::from_le_stream(bytes.into_iter()).unwrap();
        assert_eq!(device_enabled, DeviceEnabled::Disabled);
    }

    #[test]
    fn test_from_le_stream_invalid() {
        let bytes = vec![0x02];
        let device_enabled = DeviceEnabled::from_le_stream(bytes.into_iter());
        assert!(device_enabled.is_none());
    }

    #[test]
    fn test_enabled_to_le_stream() {
        let device_enabled = DeviceEnabled::Enabled;
        let bytes: Vec<u8> = device_enabled.to_le_stream().collect();
        assert_eq!(bytes, vec![0x01]);
    }

    #[test]
    fn test_disabled_to_le_stream() {
        let device_enabled = DeviceEnabled::Disabled;
        let bytes: Vec<u8> = device_enabled.to_le_stream().collect();
        assert_eq!(bytes, vec![0x00]);
    }
}
