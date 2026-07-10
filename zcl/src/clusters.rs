//! Cluster groups.

use self::general::{alarms, basic, groups, identify, level, on_off, scenes};
use self::lighting::color_control;
use crate::{Header, ParseFrameError, Scope};

pub mod general;
pub mod global;
pub mod ias;
pub mod lighting;
pub mod measurement_and_sensing;

/// Available ZCL clusters.
// TODO: Add all ZCL clusters.
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

    /// Alarms cluster commands.
    Alarms(alarms::Command),

    /// Scenes cluster commands.
    Scenes(scenes::Command),

    /// Color Control cluster commands.
    ColorControl(color_control::Command),

    /// IAS Zone cluster commands.
    IasZone(ias::zone::Command),
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
                <basic::Command as zb_core::ClusterSpecific>::ID => {
                    basic::Command::parse_zcl_frame(header, bytes).map(Self::Basic)
                }
                <groups::Command as zb_core::ClusterSpecific>::ID => {
                    groups::Command::parse_zcl_frame(header, bytes).map(Self::Groups)
                }
                <identify::Command as zb_core::ClusterSpecific>::ID => {
                    identify::Command::parse_zcl_frame(header, bytes).map(Self::Identify)
                }
                <on_off::Command as zb_core::ClusterSpecific>::ID => {
                    on_off::Command::parse_zcl_frame(header, bytes).map(Self::OnOff)
                }
                <level::Command as zb_core::ClusterSpecific>::ID => {
                    level::Command::parse_zcl_frame(header, bytes).map(Self::Level)
                }
                <alarms::Command as zb_core::ClusterSpecific>::ID => {
                    alarms::Command::parse_zcl_frame(header, bytes).map(Self::Alarms)
                }
                <scenes::Command as zb_core::ClusterSpecific>::ID => {
                    scenes::Command::parse_zcl_frame(header, bytes).map(Self::Scenes)
                }
                <color_control::Command as zb_core::ClusterSpecific>::ID => {
                    color_control::Command::parse_zcl_frame(header, bytes).map(Self::ColorControl)
                }
                <ias::zone::Command as zb_core::ClusterSpecific>::ID => {
                    ias::zone::Command::parse_zcl_frame(header, bytes).map(Self::IasZone)
                }
                invalid_cluster_id => Err(ParseFrameError::InvalidClusterId(invalid_cluster_id)),
            },
        }
    }
}
