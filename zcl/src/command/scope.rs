use zigbee::Cluster;

use crate::Scope;

/// Trait to associate a scope with
pub trait Scoped {
    /// The scope of the command.
    const SCOPE: Scope;
}

impl<T> Scoped for T
where
    T: Cluster,
{
    const SCOPE: Scope = Scope::ClusterSpecific;
}
