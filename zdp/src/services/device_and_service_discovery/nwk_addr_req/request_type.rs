use num_enum::{IntoPrimitive, TryFromPrimitive};

/// The type of NWK Address Request.
#[derive(
    Clone, Copy, Debug, Eq, Hash, IntoPrimitive, Ord, PartialEq, PartialOrd, TryFromPrimitive,
)]
#[repr(u8)]
pub enum RequestType {
    /// Request for a single device response.
    SingleDeviceResponse = 0x00,
    /// Request for an extended response.
    ExtendedResponse = 0x01,
}
