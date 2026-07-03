use apis_saltans_core::{ClusterId, Direction};

use crate::macros::zcl_command;

zcl_command! {
    /// Reset a device to factory defaults.
    ResetToFactoryDefaults {
        { ClusterId::Basic } => Basic;
        command_id: 0x00;
        direction: Direction::ClientToServer;
        => super::ResetToFactoryDefaults;
        derive(Default);
        fields;
    }
}
