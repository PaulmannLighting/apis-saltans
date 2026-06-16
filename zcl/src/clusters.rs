//! Cluster groups.

use alloc::boxed::Box;
use core::error::Error;
use core::fmt::Display;

use le_stream::ToLeStream;
use zigbee::Direction;

use self::general::{basic, groups, identify, level, on_off};
use self::lighting::color_control;
use crate::{CommandDispatch, Header, ParseFrameError, Scope};

pub mod general;
pub mod global;
pub mod lighting;
pub mod measurement_and_sensing;

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

impl CommandDispatch for Cluster {
    fn command_id(&self) -> u8 {
        match self {
            Self::Global(cmd) => cmd.command_id(),
            Self::Basic(cmd) => cmd.command_id(),
            Self::Groups(cmd) => cmd.command_id(),
            Self::Identify(cmd) => cmd.command_id(),
            Self::OnOff(cmd) => cmd.command_id(),
            Self::Level(cmd) => cmd.command_id(),
            Self::ColorControl(cmd) => cmd.command_id(),
        }
    }

    fn scope(&self) -> Scope {
        match self {
            Self::Global(cmd) => cmd.scope(),
            Self::Basic(cmd) => cmd.scope(),
            Self::Groups(cmd) => cmd.scope(),
            Self::Identify(cmd) => cmd.scope(),
            Self::OnOff(cmd) => cmd.scope(),
            Self::Level(cmd) => cmd.scope(),
            Self::ColorControl(cmd) => cmd.scope(),
        }
    }

    fn direction(&self) -> Direction {
        match self {
            Self::Global(cmd) => cmd.direction(),
            Self::Basic(cmd) => cmd.direction(),
            Self::Groups(cmd) => cmd.direction(),
            Self::Identify(cmd) => cmd.direction(),
            Self::OnOff(cmd) => cmd.direction(),
            Self::Level(cmd) => cmd.direction(),
            Self::ColorControl(cmd) => cmd.direction(),
        }
    }

    fn disable_default_response(&self) -> bool {
        match self {
            Self::Global(cmd) => cmd.disable_default_response(),
            Self::Basic(cmd) => cmd.disable_default_response(),
            Self::Groups(cmd) => cmd.disable_default_response(),
            Self::Identify(cmd) => cmd.disable_default_response(),
            Self::OnOff(cmd) => cmd.disable_default_response(),
            Self::Level(cmd) => cmd.disable_default_response(),
            Self::ColorControl(cmd) => cmd.disable_default_response(),
        }
    }
}

impl ToLeStream for Cluster {
    type Iter = Iter;

    fn to_le_stream(self) -> Self::Iter {
        match self {
            Self::Global(cmd) => Iter::Global(cmd.to_le_stream()),
            Self::Basic(cmd) => Iter::Basic(cmd.to_le_stream()),
            Self::Groups(cmd) => Iter::Groups(cmd.to_le_stream().into()),
            Self::Identify(cmd) => Iter::Identify(cmd.to_le_stream()),
            Self::OnOff(cmd) => Iter::OnOff(cmd.to_le_stream()),
            Self::Level(cmd) => Iter::Level(cmd.to_le_stream()),
            Self::ColorControl(cmd) => Iter::ColorControl(cmd.to_le_stream()),
        }
    }
}

#[derive(Debug)]
pub enum Iter {
    Global(<global::Command as ToLeStream>::Iter),
    Basic(<basic::Command as ToLeStream>::Iter),
    Groups(Box<<groups::Command as ToLeStream>::Iter>),
    Identify(<identify::Command as ToLeStream>::Iter),
    OnOff(<on_off::Command as ToLeStream>::Iter),
    Level(<level::Command as ToLeStream>::Iter),
    ColorControl(<color_control::Command as ToLeStream>::Iter),
}

impl Iterator for Iter {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Global(iter) => iter.next(),
            Self::Basic(iter) => iter.next(),
            Self::Groups(iter) => iter.next(),
            Self::Identify(iter) => iter.next(),
            Self::OnOff(iter) => iter.next(),
            Self::Level(iter) => iter.next(),
            Self::ColorControl(iter) => iter.next(),
        }
    }
}
