use crate::zcl::{Cluster, Command};

/// Command to move a light to a specific saturation.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct MoveToSaturation {
    saturation: u8,
    transition_time: u16,
}

impl MoveToSaturation {
    #[must_use]
    pub const fn new(saturation: u8, transition_time: u16) -> Self {
        Self {
            saturation,
            transition_time,
        }
    }

    #[must_use]
    pub const fn saturation(self) -> u8 {
        self.saturation
    }

    #[must_use]
    pub const fn transition_time(self) -> u16 {
        self.transition_time
    }
}

impl Cluster for MoveToSaturation {
    const ID: u16 = 0x0300;
}

impl Command for MoveToSaturation {
    const ID: u8 = 0x03;
}
