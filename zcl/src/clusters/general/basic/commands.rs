//! Commands for the Basic cluster.

use zigbee::Cluster;
use zigbee_macros::ParseZclFrame;

pub use self::reset_to_factory_defaults::ResetToFactoryDefaults;

mod reset_to_factory_defaults;

/// Available commands for the Basic cluster.
#[derive(Clone, Debug, Eq, PartialEq, Hash, ParseZclFrame)]
pub enum Command {
    /// Reset to Factory Defaults command.
    ResetToFactoryDefaults(ResetToFactoryDefaults),
}

impl Cluster for Command {
    const ID: u16 = super::CLUSTER_ID;
}
