use apis_saltans_core::{Cluster, Direction};

use crate::macros::zcl_command;

zcl_command! {
    /// Request the target to respond if they are currently identifying themselves.
    #[repr(transparent)]
    IdentifyQuery {
        { Cluster::Identify } => Identify;
        command_id: 0x01;
        direction: Direction::ClientToServer;
        derive(Default);
        fields;
    }
}
