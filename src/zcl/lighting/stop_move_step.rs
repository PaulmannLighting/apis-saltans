use crate::zcl::Command;
use crate::zcl::lighting::Lighting;

/// Command to stop a move step in a lighting device.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct StopMoveStep;

impl Lighting for StopMoveStep {}

impl Command for StopMoveStep {
    const ID: u8 = 47;
}
