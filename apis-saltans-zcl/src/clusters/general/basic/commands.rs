//! Commands for the Basic cluster.

use apis_saltans_core::ClusterId;

pub use self::reset_to_factory_defaults::ResetToFactoryDefaults;
use crate::macros::zcl_command_enum;

mod reset_to_factory_defaults;

// Available commands for the Basic cluster.
zcl_command_enum! {
    { ClusterId::Basic } => Basic;
    ResetToFactoryDefaults(ResetToFactoryDefaults),
}
