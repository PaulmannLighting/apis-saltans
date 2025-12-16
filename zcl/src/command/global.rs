use zigbee::Direction;

use crate::{ClusterDirected, Command, Scope};

/// Trait to identify a Zigbee global command.
pub trait Global {
    /// The command identifier.
    const ID: u8;

    /// The command direction.
    const DIRECTION: Direction;

    /// Whether to disable the client response for this command.
    const DISABLE_CLIENT_RESPONSE: bool = false;

    /// The manufacturer code for this command, if any.
    const MANUFACTURER_CODE: Option<u16> = None;

    /// Direct this global command to a specific cluster.
    #[must_use]
    fn for_cluster(self, cluster_id: u16) -> ClusterDirected<Self>
    where
        Self: Sized,
    {
        ClusterDirected::new(cluster_id, self)
    }
}

impl<T> Command for T
where
    T: Global,
{
    const ID: u8 = T::ID;
    const DIRECTION: Direction = T::DIRECTION;
    const SCOPE: Scope = Scope::Global;
    const DISABLE_CLIENT_RESPONSE: bool = T::DISABLE_CLIENT_RESPONSE;
    const MANUFACTURER_CODE: Option<u16> = T::MANUFACTURER_CODE;
}
