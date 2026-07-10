use zb_core::ClusterSpecific;

use crate::Scope;

/// Trait to associate a scope with a command.
pub trait Scoped {
    /// The scope of the command.
    const SCOPE: Scope;
}

impl<T> Scoped for T
where
    T: ClusterSpecific,
{
    const SCOPE: Scope = Scope::ClusterSpecific;
}
