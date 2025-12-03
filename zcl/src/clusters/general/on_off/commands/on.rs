use le_stream::ToLeStream;
use zigbee::{Cluster, Command};

use crate::general::on_off::CLUSTER_ID;

/// Switch a device on.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, ToLeStream)]
pub struct On;

impl Cluster for On {
    const ID: u16 = CLUSTER_ID;
}

impl Command for On {
    const ID: u8 = 0x01;
}
