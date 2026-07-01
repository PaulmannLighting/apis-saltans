//! Writeable attributes for the On/Off cluster.

use apis_saltans_core::types::{Bool, Uint16};
use repr_discriminant::ReprDiscriminant;

use super::StartUpOnOff;

/// Readable attributes for the On/Off cluster.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(u16)]
#[derive(ReprDiscriminant)]
pub enum Attribute {
    /// On/Off state of the device.
    OnOff(Bool) = 0x0000,
    ///  On time attribute.
    OnTime(Uint16) = 0x4001,
    ///  Off wait time attribute.
    OffWaitTime(Uint16) = 0x4002,
    ///  Behavoir of the On/Off cluster at startup.
    StartUpOnOff(StartUpOnOff) = 0x4003,
}
