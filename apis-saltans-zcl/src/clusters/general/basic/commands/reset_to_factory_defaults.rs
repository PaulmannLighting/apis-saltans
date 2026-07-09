use apis_saltans_core::{Cluster, Direction};

use crate::macros::zcl_command;

zcl_command! {
    /// Reset a device to factory defaults.
    ResetToFactoryDefaults {
        { Cluster::Basic } => Basic;
        command_id: 0x00;
        direction: Direction::ClientToServer;
        derive(Default);
        fields;
    }
}
