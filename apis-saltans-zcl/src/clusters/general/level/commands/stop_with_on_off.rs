use apis_saltans_core::{ClusterId, Direction};

use crate::macros::zcl_command;
use crate::options::Options;

zcl_command! {
    /// Stop command.
    StopWithOnOff {
        { ClusterId::Level } => Level;
        command_id: 0x07;
        direction: Direction::ClientToServer;
        => super::StopWithOnOff;
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
