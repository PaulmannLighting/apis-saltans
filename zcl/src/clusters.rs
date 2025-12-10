//! Cluster groups.

use zigbee_macros::ParseZclCluster;

use crate::general::{basic, groups, identify, on_off};

pub mod general;
pub mod lighting;

/// Available ZCL clusters.
// TODO: Add all ZCL clusters.
#[expect(clippy::large_enum_variant)]
#[derive(Clone, Debug, Eq, PartialEq, Hash, ParseZclCluster)]
pub enum Cluster {
    /// Basic cluster commands.
    Basic(basic::Command),
    /// Groups cluster commands.
    Groups(groups::Command),
    /// Identify cluster commands.
    Identify(identify::Command),
    /// On/Off cluster commands.
    OnOff(on_off::Command),
}
