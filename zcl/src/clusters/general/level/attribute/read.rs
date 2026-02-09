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
    CurrentLevel(Uint8) = 0x0000,
    RemainingTime(u16) = 0x0001,
    MinLevel(Uint8) = 0x0002,
    MaxLevel(Uint8) = 0x0003,
    CurrentFrequency(u16) = 0x0004,
    MinFrequency(u16) = 0x0005,
    MaxFrequency(u16) = 0x0006,
    OnOffTransitionTime(u16) = 0x0010,
    OnLevel(Uint8) = 0x0011,
    OnTransitionTime(Uint16) = 0x0012,
    OffTransitionTime(Uint16) = 0x0013,
    DefaultMoveRate(Uint8) = 0x0014,
    Options(Options) = 0x000F,
    StartUpCurrentLevel(u8) = 0x4000,
}
