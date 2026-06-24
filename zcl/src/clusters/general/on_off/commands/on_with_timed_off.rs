use le_stream::{FromLeStream, ToLeStream};
use zigbee::types::Uint16;
use zigbee::{ClusterId, ClusterSpecific, Direction};

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
    pub const fn new(on_off_control: OnOffControl, on_time: Uint16, off_wait_time: Uint16) -> Self {
        Self {
            on_off_control,
            on_time,
            off_wait_time,
        }
    }

    /// Return the on/off control field.
    #[must_use]
    pub const fn on_off_control(&self) -> OnOffControl {
        self.on_off_control
    }

    /// Return the on time, if any, in deciseconds.
    #[must_use]
    pub fn on_time(&self) -> Option<u16> {
        self.on_time.into()
    }

    /// Return the off wait time, if any, in deciseconds.
    #[must_use]
    pub fn off_wait_time(&self) -> Option<u16> {
        self.off_wait_time.into()
    }
}

impl ClusterSpecific for OnWithTimedOff {
    const CLUSTER: ClusterId = ClusterId::OnOff;
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
