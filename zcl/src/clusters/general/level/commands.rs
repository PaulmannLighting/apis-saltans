pub use step::Step;
use zigbee::Cluster;
use zigbee_macros::ParseZclFrame;

use crate::CommandId;

mod step;

/// Available On/Off cluster commands.
#[derive(Clone, Debug, Eq, PartialEq, Hash, ParseZclFrame)]
pub enum Command {
    /// Step command.
    Step(Step),
}

impl Cluster for Command {
    const ID: u16 = super::CLUSTER_ID;
}

impl CommandId for Command {
    fn command_id(&self) -> u8 {
        match self {
            Self::Step(cmd) => cmd.command_id(),
        }
    }
}
