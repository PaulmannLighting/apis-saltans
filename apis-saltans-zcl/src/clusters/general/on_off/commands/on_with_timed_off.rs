use le_stream::{FromLeStream, ToLeStream};
use apis_saltans_core::types::Uint16;
use apis_saltans_core::units::Deciseconds;
use apis_saltans_core::{ClusterId, Cluster, Direction};

pub use self::on_off_control::OnOffControl;
use crate::Command;

mod on_off_control;

/// Command to turn on a device for a specified time, then turn it off after a wait period.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
pub struct OnWithTimedOff {
    on_off_control: OnOffControl,
    on_time: Uint16,
    off_wait_time: Uint16,
}

impl OnWithTimedOff {
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

impl Cluster<ClusterId> for OnWithTimedOff {
    const ID: ClusterId = ClusterId::OnOff;
}

impl Command for OnWithTimedOff {
    const ID: u8 = 0x42;
    const DIRECTION: Direction = Direction::ClientToServer;
}

impl From<OnWithTimedOff> for crate::Cluster {
    fn from(command: OnWithTimedOff) -> Self {
        Self::OnOff(command.into())
    }
}
