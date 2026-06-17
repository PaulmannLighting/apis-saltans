use serde::{Deserialize, Serialize};

use crate::Device;

/// The persistent state.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct State {
    /// The devices in the network.
    pub devices: Box<[Device]>,
}
