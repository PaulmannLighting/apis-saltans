//! Binding management.

use macaddr::MacAddr8;
use zdp::Destination;
use zigbee::Endpoint;

use crate::Error;

/// Trait for binding management operations.
pub trait Binding {
    /// Create a binding for the specified cluster ID to the given destination.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if execution of the command failed.
    fn bind(
        &self,
        src_address: MacAddr8,
        src_endpoint: Endpoint,
        cluster_id: u16,
        destination: Destination,
    ) -> impl Future<Output = Result<u8, Error>> + Send;

    /// Remove a binding for the specified cluster ID to the given destination.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if execution of the command failed.
    fn unbind(
        &self,
        src_address: MacAddr8,
        src_endpoint: Endpoint,
        cluster_id: u16,
        destination: Destination,
    ) -> impl Future<Output = Result<u8, Error>> + Send;
}
