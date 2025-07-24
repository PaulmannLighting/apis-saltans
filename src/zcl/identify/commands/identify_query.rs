use crate::zcl::identify::CLUSTER_ID;
use crate::zcl::{Cluster, Command};

/// Request the target to respond if they are currently identifying themselves.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct IdentifyQuery;

impl Cluster for IdentifyQuery {
    const ID: u16 = CLUSTER_ID;
}

impl Command for IdentifyQuery {
    const ID: u8 = 0x01;
}
