//! Cluster groups.

use zigbee::Direction;

use crate::ParseFrameError;
use crate::general::{basic, groups, identify, on_off};

pub mod general;
pub mod lighting;

/// Available ZCL frames.
// TODO: Add all ZCL commands.
#[expect(clippy::large_enum_variant)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[repr(u16)]
pub enum Cluster {
    /// Basic cluster commands.
    Basic(basic::Command) = basic::CLUSTER_ID,
    /// Groups cluster commands.
    Groups(groups::Command) = groups::CLUSTER_ID,
    /// Identify cluster commands.
    Identify(identify::Command) = identify::CLUSTER_ID,
    /// On/Off cluster commands.
    OnOff(on_off::Command) = on_off::CLUSTER_ID,
}

impl Cluster {
    pub(crate) fn from_le_stream<T>(
        cluster_id: u16,
        command_id: u8,
        direction: Direction,
        bytes: T,
    ) -> Result<Self, ParseFrameError>
    where
        T: Iterator<Item = u8>,
    {
        match cluster_id {
            basic::CLUSTER_ID => {
                basic::Command::from_le_stream(command_id, direction, bytes).map(Self::Basic)
            }
            groups::CLUSTER_ID => {
                groups::Command::from_le_stream(command_id, direction, bytes).map(Self::Groups)
            }
            identify::CLUSTER_ID => {
                identify::Command::from_le_stream(command_id, direction, bytes).map(Self::Identify)
            }
            on_off::CLUSTER_ID => {
                on_off::Command::from_le_stream(command_id, direction, bytes).map(Self::OnOff)
            }
            other => Err(ParseFrameError::InvalidClusterId(other)),
        }
    }
}
