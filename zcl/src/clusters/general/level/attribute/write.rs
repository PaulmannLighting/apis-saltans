//! Writable attributes for the Level cluster.

use le_stream::FromLeStreamTagged;
use repr_discriminant::ReprDiscriminant;
use zigbee::types::{Uint8, Uint16};

use super::{Options, read};

/// Writable attributes for the Level cluster.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(u16)]
#[derive(ReprDiscriminant, FromLeStreamTagged)]
pub enum Attribute {
    /// On/Off transition time, in tenths of a second.
    OnOffTransitionTime(u16) = 0x0010,
    /// Level to move to when the device is turned on.
    OnLevel(Uint8) = 0x0011,
    /// Time, in tenths of a second, until the device reaches the `On` state when turned on.
    OnTransitionTime(Uint16) = 0x0012,
    /// Time, in tenths of a second, until the device reaches the `Off` state when turned off.
    OffTransitionTime(Uint16) = 0x0013,
    /// Rate of change when the device is moving to a new level, in levels per second.
    DefaultMoveRate(Uint8) = 0x0014,
    /// Bitmask of options for the device.
    Options(Options) = 0x000F,
    /// Level to move to when the device is turned on, if the previous level is not known.
    StartUpCurrentLevel(u8) = 0x4000,
}

impl TryFrom<read::Attribute> for Attribute {
    type Error = read::Attribute;

    fn try_from(read: read::Attribute) -> Result<Self, Self::Error> {
        match read {
            read::Attribute::OnOffTransitionTime(time) => Ok(Self::OnOffTransitionTime(time)),
            read::Attribute::OnLevel(level) => Ok(Self::OnLevel(level)),
            read::Attribute::OnTransitionTime(time) => Ok(Self::OnTransitionTime(time)),
            read::Attribute::OffTransitionTime(time) => Ok(Self::OffTransitionTime(time)),
            read::Attribute::DefaultMoveRate(rate) => Ok(Self::DefaultMoveRate(rate)),
            read::Attribute::Options(options) => Ok(Self::Options(options)),
            read::Attribute::StartUpCurrentLevel(level) => Ok(Self::StartUpCurrentLevel(level)),
            other => Err(other),
        }
    }
}
