use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

/// The logical type of Zigbee device.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, FromPrimitive)]
#[repr(u8)]
pub enum LogicalType {
    /// The device is a coordinator.
    Coordinator = 0b000,
    /// The device is a router.
    Router = 0b001,
    /// The device is an end device.
    EndDevice = 0b010,
}

impl TryFrom<u8> for LogicalType {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or(value)
    }
}

impl From<LogicalType> for u8 {
    fn from(logical_type: LogicalType) -> Self {
        logical_type as Self
    }
}
