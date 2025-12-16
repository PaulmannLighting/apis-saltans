use zigbee::{ClusterId, Direction};

use crate::{Command, Scope};

/// A command which is directed towards a specific cluster.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ClusterDirected<T> {
    cluster_id: u16,
    payload: T,
}

impl<T> ClusterDirected<T> {
    /// Create a new `ClusterDirected` command.
    #[must_use]
    pub(crate) const fn new(cluster_id: u16, payload: T) -> Self {
        Self {
            cluster_id,
            payload,
        }
    }
}

impl<T> ClusterId for ClusterDirected<T> {
    fn cluster_id(&self) -> u16 {
        self.cluster_id
    }
}

impl<T> Command for ClusterDirected<T>
where
    T: Command,
{
    const ID: u8 = T::ID;
    const DIRECTION: Direction = T::DIRECTION;
    const SCOPE: Scope = T::SCOPE;
    const DISABLE_CLIENT_RESPONSE: bool = T::DISABLE_CLIENT_RESPONSE;
    const MANUFACTURER_CODE: Option<u16> = T::MANUFACTURER_CODE;
}
