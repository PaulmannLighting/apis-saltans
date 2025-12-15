use le_stream::{FromLeStream, ToLeStream};
use zigbee::{Cluster, Command, Direction};

use crate::clusters::general::on_off::CLUSTER_ID;

/// Switch a device off.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
pub struct Off;

impl Cluster for Off {
    const ID: u16 = CLUSTER_ID;
}

impl Command for Off {
    const ID: u8 = 0x00;
    const DIRECTION: Direction = Direction::ClientToServer;
}
