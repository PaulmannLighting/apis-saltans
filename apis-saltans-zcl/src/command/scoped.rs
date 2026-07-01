use apis_saltans_core::Cluster;

use crate::Scope;

/// Trait to associate a scope with a command.
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
