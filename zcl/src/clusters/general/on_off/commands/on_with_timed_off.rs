use zb_core::types::Uint16;
use zb_core::units::Deciseconds;
use zb_core::{Cluster, Direction};

pub use self::on_off_control::OnOffControl;
use crate::macros::zcl_command;

mod on_off_control;

zcl_command! {
    /// Command to turn on a device for a specified time, then turn it off after a wait period.
    OnWithTimedOff {
        { Cluster::OnOff } => OnOff;
        command_id: 0x42;
        direction: Direction::ClientToServer;
        derive(Default);
        fields {
            on_off_control: OnOffControl,
            on_time: Uint16,
            off_wait_time: Uint16,
        }

        constructor {
            /// Create a new `OnWithTimedOff` command.
            #[must_use]
            pub const fn new(
                on_off_control: OnOffControl,
                on_time: Deciseconds,
                off_wait_time: Deciseconds,
            ) -> Self {
                Self {
                    on_off_control,
                    on_time: on_time.into_inner(),
                    off_wait_time: off_wait_time.into_inner(),
                }
            }
        }

        getters {
            /// Return the on/off control field.
            #[must_use]
            pub const fn on_off_control(&self) -> OnOffControl {
                self.on_off_control
            }

            /// Return the on time, if any, in deciseconds.
            #[must_use]
            pub fn on_time(&self) -> Option<Deciseconds> {
                self.on_time.try_into().ok()
            }

            /// Return the off wait time, if any, in deciseconds.
            #[must_use]
            pub fn off_wait_time(&self) -> Option<Deciseconds> {
                self.off_wait_time.try_into().ok()
            }
        }
    }
}
