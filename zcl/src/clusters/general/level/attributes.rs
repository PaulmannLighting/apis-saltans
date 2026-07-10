//! Attributes of the Level Control cluster.

use apis_saltans_core::Cluster;
use apis_saltans_core::types::{Uint8, Uint16};

pub use self::types::Options;
use crate::macros::zcl_attributes;

mod types;

zcl_attributes! {
    cluster: Cluster::Level;

    /// Current level of the device.
    CurrentLevel = 0x0000: Uint8 { R, P },
    /// Time, in tenths of a second, until the current level is reached.
    RemainingTime = 0x0001: Uint16 { R },
    /// Minimum level of the device.
    MinLevel = 0x0002: Uint8 { R },
    /// Maximum level of the device.
    MaxLevel = 0x0003: Uint8 { R },
    /// Current frequency of the device.
    CurrentFrequency = 0x0004: Uint16 { R, P },
    /// Minimum frequency of the device.
    MinFrequency = 0x0005: Uint16 { R },
    /// Maximum frequency of the device.
    MaxFrequency = 0x0006: Uint16 { R },
    /// Bitmask of options for the device.
    Options = 0x000f: Options { R, W },
    /// On/Off transition time, in tenths of a second.
    OnOffTransitionTime = 0x0010: Uint16 { R, W },
    /// Level to move to when the device is turned on.
    OnLevel = 0x0011: Uint8 { R, W },
    /// Time, in tenths of a second, until the device reaches the `On` state when turned on.
    OnTransitionTime = 0x0012: Uint16 { R, W },
    /// Time, in tenths of a second, until the device reaches the `Off` state when turned off.
    OffTransitionTime = 0x0013: Uint16 { R, W },
    /// Rate of change when the device is moving to a new level, in levels per second.
    DefaultMoveRate = 0x0014: Uint8 { R, W },
    /// Level to move to when the device is turned on, if the previous level is not known.
    StartUpCurrentLevel = 0x4000: Uint8 { R, W },
}
