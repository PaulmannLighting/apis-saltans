//! Writable attributes for the Level cluster.

use le_stream::FromLeStreamTagged;
use repr_discriminant::ReprDiscriminant;
use zigbee::types::{Uint8, Uint16};

use super::Options;

/// Writable attributes for the Level cluster.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(u16)]
#[derive(ReprDiscriminant, FromLeStreamTagged)]
pub enum Attribute {
    OnOffTransitionTime(u16) = 0x0010,
    OnLevel(Uint8) = 0x0011,
    OnTransitionTime(Uint16) = 0x0012,
    OffTransitionTime(Uint16) = 0x0013,
    DefaultMoveRate(Uint8) = 0x0014,
    Options(Options) = 0x000F,
    StartUpCurrentLevel(u8) = 0x4000,
}
