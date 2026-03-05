//! Writeable attributes for the On/Off cluster.

use le_stream::FromLeStreamTagged;
use repr_discriminant::ReprDiscriminant;
use zigbee::Parsable;
use zigbee::types::{Bool, Uint16};

use super::StartUpOnOff;

/// Readable attributes for the On/Off cluster.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(u16)]
#[derive(ReprDiscriminant, FromLeStreamTagged)]
pub enum Attribute {
    /// On/Off state of the device.
    OnOff(Bool) = 0x0000,
    ///  On time attribute.
    OnTime(Uint16) = 0x4001,
    ///  Off wait time attribute.
    OffWaitTime(Uint16) = 0x4002,
    ///  Start up on off attribute.
    StartUpOnOff(Parsable<u8, StartUpOnOff>) = 0x4003,
}
