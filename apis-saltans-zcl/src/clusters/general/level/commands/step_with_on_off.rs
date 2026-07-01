use le_stream::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;
use apis_saltans_core::types::Uint16;
use apis_saltans_core::units::Deciseconds;
use apis_saltans_core::{ClusterId, ClusterSpecific, Direction};

use crate::Command;
use crate::general::level::Mode;
use crate::options::Options;

/// Step with on/off command.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
pub struct StepWithOnOff {
    mode: u8,
    size: u8,
    transition_time: Uint16,
    options: Options,
}

impl StepWithOnOff {
    /// Creates a new `StepWithOnOff` command.
    #[must_use]
    pub const fn new(mode: Mode, size: u8, transition_time: Deciseconds, options: Options) -> Self {
        Self {
            mode: mode as u8,
            size,
            transition_time: transition_time.into_inner(),
            options,
        }
    }

    /// Get the mode.
    ///
    /// # Errors
    ///
    /// Returns the raw mode value if it is invalid.
    pub fn mode(self) -> Result<Mode, u8> {
        Mode::from_u8(self.mode).ok_or(self.mode)
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

impl ClusterSpecific for StepWithOnOff {
    const CLUSTER: ClusterId = ClusterId::Level;
}

impl Command for StepWithOnOff {
    const ID: u8 = 0x06;
    const DIRECTION: Direction = Direction::ClientToServer;
}

impl From<StepWithOnOff> for crate::Cluster {
    fn from(command: StepWithOnOff) -> Self {
        Self::Level(command.into())
    }
}
