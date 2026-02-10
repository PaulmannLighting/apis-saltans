//! Cluster groups.

use self::general::{basic, groups, identify, level, on_off};
use self::lighting::color_control;
use crate::{CommandId, Header, ParseFrameError, Scope};

pub mod general;
pub mod global;
pub mod lighting;
mod measurement_and_sensing;

/// Available ZCL clusters.
// TODO: Add all ZCL clusters.
#[expect(clippy::large_enum_variant)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Cluster {
    /// General cluster commands.
    Global(global::Command),
    /// Basic cluster commands.
    Basic(basic::Command),
    /// Groups cluster commands.
    Groups(groups::Command),
    /// Identify cluster commands.
    Identify(identify::Command),
    /// On/Off cluster commands.
    OnOff(on_off::Command),
    /// Level commands.
    Level(level::Command),
    /// Color Control cluster commands.
    ColorControl(color_control::Command),
}

impl Cluster {
    /// Parse a ZCL cluster command from the given cluster ID, header, and byte iterator.
    pub(crate) fn parse_zcl_cluster<T>(
        cluster_id: u16,
        header: Header,
        bytes: T,
    ) -> Result<Self, ParseFrameError>
    where
        T: Iterator<Item = u8>,
    {
        let typ = match header.control().typ() {
            Ok(typ) => typ,
            Err(error) => return Err(ParseFrameError::InvalidType(error)),
        };

        match typ {
            Scope::Global => global::Command::parse_zcl_frame(header, bytes).map(Self::Global),
            Scope::ClusterSpecific => match cluster_id {
                <basic::Command as zigbee::Cluster>::ID => {
                    basic::Command::parse_zcl_frame(header, bytes).map(Self::Basic)
                }
                <groups::Command as zigbee::Cluster>::ID => {
                    groups::Command::parse_zcl_frame(header, bytes).map(Self::Groups)
                }
                <identify::Command as zigbee::Cluster>::ID => {
                    identify::Command::parse_zcl_frame(header, bytes).map(Self::Identify)
                }
                <on_off::Command as zigbee::Cluster>::ID => {
                    on_off::Command::parse_zcl_frame(header, bytes).map(Self::OnOff)
                }
                <level::Command as zigbee::Cluster>::ID => {
                    level::Command::parse_zcl_frame(header, bytes).map(Self::Level)
                }
                <color_control::Command as zigbee::Cluster>::ID => {
                    color_control::Command::parse_zcl_frame(header, bytes).map(Self::ColorControl)
                }
                invalid_cluster_id => Err(ParseFrameError::InvalidClusterId(invalid_cluster_id)),
            },
        }
    }
}

impl CommandId for Cluster {
    fn command_id(&self) -> u8 {
        match self {
            Self::Global(command) => command.command_id(),
            Self::Basic(command) => command.command_id(),
            Self::Groups(command) => command.command_id(),
            Self::Identify(command) => command.command_id(),
            Self::OnOff(command) => command.command_id(),
            Self::Level(command) => command.command_id(),
            Self::ColorControl(command) => command.command_id(),
        }
    }
}
