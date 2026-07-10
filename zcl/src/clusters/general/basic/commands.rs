//! Commands for the Basic cluster.

use zb_core::Cluster;

pub use self::reset_to_factory_defaults::ResetToFactoryDefaults;
use crate::macros::zcl_command_enum;

mod reset_to_factory_defaults;

// Available commands for the Basic cluster.
zcl_command_enum! {
    { Cluster::Basic } => Basic;
    ResetToFactoryDefaults(ResetToFactoryDefaults),
}
