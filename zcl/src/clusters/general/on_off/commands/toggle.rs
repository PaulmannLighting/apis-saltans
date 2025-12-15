use le_stream::{FromLeStream, ToLeStream};
use zigbee::{Cluster, Direction};

use crate::Command;
use crate::clusters::general::on_off::CLUSTER_ID;

/// Toggle a device on/off state.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
pub struct Toggle;

impl Cluster for Toggle {
    const ID: u16 = CLUSTER_ID;
}

impl Command for Toggle {
    const ID: u8 = 0x02;
    const DIRECTION: Direction = Direction::ClientToServer;
}
