use num_enum::{IntoPrimitive, TryFromPrimitive};

/// The logical type of Zigbee device.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, IntoPrimitive, PartialEq, TryFromPrimitive)]
#[num_enum(error_type(name = u8, constructor = core::convert::identity))]
#[repr(u8)]
pub enum LogicalType {
    /// The device is a coordinator.
    Coordinator = 0b000,

    /// The device is a router.
    Router = 0b001,

    /// The device is an end device.
    EndDevice = 0b010,
}
