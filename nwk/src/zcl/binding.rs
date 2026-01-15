//! Binding management.

use macaddr::MacAddr8;
use zdp::{BindReq, Destination};

use crate::proxies::EndpointProxy;
use crate::{Error, Proxy};

/// Trait for binding management operations.
pub trait Binding {
    /// Create a binding for the specified cluster ID to the given destination.
    fn bind(
        &self,
        src_address: MacAddr8,
        cluster_id: u16,
        destination: Destination,
    ) -> impl Future<Output = Result<u8, Error>>;

    /// Remove a binding for the specified cluster ID to the given destination.
    fn unbind(
        &self,
        src_address: MacAddr8,
        cluster_id: u16,
        destination: Destination,
    ) -> impl Future<Output = Result<u8, Error>>;
}

impl<T> Binding for EndpointProxy<'_, T>
where
    T: Proxy + Sync,
{
    async fn bind(
        &self,
        src_address: MacAddr8,
        cluster_id: u16,
        destination: Destination,
    ) -> Result<u8, Error> {
        self.zdp()
            .unicast(BindReq::new(
                src_address,
                self.endpoint().into(),
                cluster_id,
                destination,
            ))
            .await
    }

    async fn unbind(
        &self,
        src_address: MacAddr8,
        cluster_id: u16,
        destination: Destination,
    ) -> Result<u8, Error> {
        self.zdp()
            .unicast(zdp::UnbindReq::new(
                src_address,
                self.endpoint().into(),
                cluster_id,
                destination,
            ))
            .await
    }
}
