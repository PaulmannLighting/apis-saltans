//! Readable attributes for the Level cluster.

use le_stream::FromLeStreamTagged;
use repr_discriminant::ReprDiscriminant;
use zigbee::types::{Uint8, Uint16};

use super::Options;

/// Readable attributes for the Level cluster.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(u16)]
#[derive(ReprDiscriminant, FromLeStreamTagged)]
pub enum Attribute {
    /// Current level of the device.
    CurrentLevel(Uint8) = 0x0000,
    /// Time, in tenths of a second, until the current level is reached.
    RemainingTime(u16) = 0x0001,
    /// Minimum level of the device.
    MinLevel(Uint8) = 0x0002,
    /// Maximum level of the device.
    MaxLevel(Uint8) = 0x0003,
    /// Current frequency of the device.
    CurrentFrequency(u16) = 0x0004,
    /// Minimum frequency of the device.
    MinFrequency(u16) = 0x0005,
    /// Maximum frequency of the device.
    MaxFrequency(u16) = 0x0006,
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
