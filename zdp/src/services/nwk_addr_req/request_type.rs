use num_derive::FromPrimitive;

/// The type of NWK Address Request.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, FromPrimitive)]
#[repr(u8)]
pub enum RequestType {
    /// Request for a single device response.
    SingleDeviceResponse = 0x00,
    /// Request for an extended response.
    ExtendedResponse = 0x01,
}
