use serde::{Deserialize, Serialize};

const MASK: u8 = 0b0000_0111;

/// The logical type of Zigbee device.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub enum LogicalType {
    /// The device is a coordinator.
    Coordinator = 0b000,
    /// The device is a router.
    Router = 0b001,
    /// The device is an end device.
    EndDevice = 0b010,
}
