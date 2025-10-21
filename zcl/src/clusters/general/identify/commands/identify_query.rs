use crate::clusters::general::identify::CLUSTER_ID;
use crate::{Cluster, Command};

/// Request the target to respond if they are currently identifying themselves.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IdentifyQuery;

impl Cluster for IdentifyQuery {
    const ID: u16 = CLUSTER_ID;
}

impl Command for IdentifyQuery {
    const ID: u8 = 0x01;
}
