//! Scene-related attributes for the On/Off cluster.

use le_stream::FromLeStream;
use repr_discriminant::ReprDiscriminant;
use apis_saltans_core::types::Bool;

/// Readable attributes for the On/Off cluster.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(u16)]
#[derive(ReprDiscriminant, FromLeStream)]
pub enum Attribute {
    /// On/Off state of the device.
    OnOff(Bool) = 0x0000,
}
