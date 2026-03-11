use le_stream::{FromLeStream, ToLeStream};
use zigbee::{Cluster, Direction};

use crate::clusters::general::on_off::CLUSTER_ID;
use crate::{Command, Native, Scope};

/// Switch a device on.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
pub struct On;

impl Cluster for On {
    const ID: u16 = CLUSTER_ID;
}

impl Command for On {
    const ID: u8 = 0x01;
    const DIRECTION: Direction = Direction::ClientToServer;
    const SCOPE: Scope = Scope::ClusterSpecific;
}

impl Native for On {}
