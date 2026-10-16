//! Cluster groups.

use zb_core::ClusterSpecific;

use self::general::{alarms, basic, groups, identify, level, on_off, ota_upgrade, scenes};
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
#[cfg_attr(
    not(feature = "alloc"),
    allow(
        clippy::large_enum_variant,
        variant_size_differences,
        reason = "allocation-free variants own bounded storage inline"
    )
)]
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

    /// OTA Upgrade cluster commands.
    OtaUpgrade(ota_upgrade::Command),

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
                <basic::Command as ClusterSpecific>::ID => {
                    basic::Command::parse_zcl_frame(header, bytes).map(Self::Basic)
                }
                <groups::Command as ClusterSpecific>::ID => {
                    groups::Command::parse_zcl_frame(header, bytes).map(Self::Groups)
                }
                <identify::Command as ClusterSpecific>::ID => {
                    identify::Command::parse_zcl_frame(header, bytes).map(Self::Identify)
                }
                <on_off::Command as ClusterSpecific>::ID => {
                    on_off::Command::parse_zcl_frame(header, bytes).map(Self::OnOff)
                }
                <level::Command as ClusterSpecific>::ID => {
                    level::Command::parse_zcl_frame(header, bytes).map(Self::Level)
                }
                <alarms::Command as ClusterSpecific>::ID => {
                    alarms::Command::parse_zcl_frame(header, bytes).map(Self::Alarms)
                }
                <scenes::Command as ClusterSpecific>::ID => {
                    scenes::Command::parse_zcl_frame(header, bytes).map(Self::Scenes)
                }
                <ota_upgrade::Command as ClusterSpecific>::ID => {
                    ota_upgrade::Command::parse_zcl_frame(header, bytes).map(Self::OtaUpgrade)
                }
                <color_control::Command as ClusterSpecific>::ID => {
                    color_control::Command::parse_zcl_frame(header, bytes).map(Self::ColorControl)
                }
                <ias::zone::Command as ClusterSpecific>::ID => {
                    ias::zone::Command::parse_zcl_frame(header, bytes).map(Self::IasZone)
                }
                invalid_cluster_id => Err(ParseFrameError::InvalidClusterId(invalid_cluster_id)),
            },
        }
    }
}
