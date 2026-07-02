use serde::{Deserialize, Serialize};

pub use self::attributes::Attributes;
pub use self::device::Device;
pub use self::endpoint::Endpoint;

mod attributes;
mod device;
mod endpoint;

/// The persistent state.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct State {
    /// The devices in the network.
    pub devices: Box<[Device]>,
}
