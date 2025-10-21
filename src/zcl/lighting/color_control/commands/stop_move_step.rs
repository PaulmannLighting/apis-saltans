use crate::zcl::lighting::color_control::CLUSTER_ID;
use crate::{Cluster, Command};

/// Command to stop a move step in a lighting device.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct StopMoveStep;

impl Cluster for StopMoveStep {
    const ID: u16 = CLUSTER_ID;
}

impl Command for StopMoveStep {
    const ID: u8 = 47;
}
