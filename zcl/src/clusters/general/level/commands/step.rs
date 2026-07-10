use zb_core::types::Uint16;
use zb_core::units::Deciseconds;
use zb_core::{Cluster, Direction};

use crate::level::Mode;
use crate::macros::zcl_command;
use crate::options::Options;

zcl_command! {
    /// Step command.
    Step {
        { Cluster::Level } => Level;
        command_id: 0x02;
        direction: Direction::ClientToServer;
        derive(Copy);
        fields {
            mode: u8,
            size: u8,
            transition_time: Uint16,
            options: Options,
        }

        constructor {
            /// Creates a new `Step` command.
            #[must_use]
            pub fn new(mode: Mode<u8>, transition_time: Deciseconds, options: Options) -> Self {
                Self {
                    mode: mode.discriminant(),
                    size: mode.into_stride(),
                    transition_time: transition_time.into_inner(),
                    options,
                }
            }
        }

        getters {
            /// Get the mode.
            #[must_use]
            pub fn mode(self) -> Option<Mode<u8>> {
                Mode::new(self.mode, self.size()).ok()
            }

            /// Get the size.
            #[must_use]
            pub const fn size(self) -> u8 {
                self.size
            }

            /// Return the transition time, if any, in deciseconds.
            #[must_use]
            pub fn transition_time(self) -> Option<Deciseconds> {
                self.transition_time.try_into().ok()
            }

            /// Get the options.
            #[must_use]
            pub const fn options(self) -> Options {
                self.options
            }
        }
    }
}
