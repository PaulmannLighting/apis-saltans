use zb_core::{Cluster, Direction};

use crate::Options;
use crate::macros::zcl_command;

zcl_command! {
    /// Command to move a light's color.
    MoveColor {
        { Cluster::ColorControl } => ColorControl;
        command_id: 0x08;
        direction: Direction::ClientToServer;
        derive(Copy);
        fields {
            rate_x: i16,
            rate_y: i16,
            options: Options,
        }

        getters {
            /// Return the rate of change in the X color component.
            #[must_use]
            pub const fn rate_x(&self) -> i16 {
                self.rate_x
            }

            /// Return the rate of change in the Y color component.
            #[must_use]
            pub const fn rate_y(&self) -> i16 {
                self.rate_y
            }

            /// Return the options for the command.
            #[must_use]
            pub const fn options(&self) -> Options {
                self.options
            }
        }
    }
}
