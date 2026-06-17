use serde::{Deserialize, Serialize};

use crate::Device;

/// The persistent state.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct State {
    devices: Box<[Device]>,
}
