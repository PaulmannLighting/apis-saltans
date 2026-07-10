use zb_core::{Cluster, Direction};

use crate::macros::zcl_command;
use crate::options::Options;

zcl_command! {
    /// Stop command.
    StopWithOnOff {
        { Cluster::Level } => Level;
        command_id: 0x07;
        direction: Direction::ClientToServer;
        derive(Default);
        fields {
            options: Options,
        }

        getters {
            /// Get the options.
            #[must_use]
            pub const fn options(self) -> Options {
                self.options
            }
        }
    }
}
