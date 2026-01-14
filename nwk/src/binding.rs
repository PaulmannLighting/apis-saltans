//! Binding management.

use zdp::{BindReq, Destination};

use crate::endpoint_proxy::EndpointProxy;
use crate::{Error, Proxy};

/// Trait for binding management operations.
pub trait Binding {
    /// Create a binding for the specified cluster ID to the given destination.
    fn bind(
        &self,
        cluster_id: u16,
        destination: Destination,
    ) -> impl Future<Output = Result<u8, Error>>;

    /// Remove a binding for the specified cluster ID to the given destination.
    fn unbind(
        &self,
        cluster_id: u16,
        destination: Destination,
    ) -> impl Future<Output = Result<u8, Error>>;
}

impl<T> Binding for EndpointProxy<'_, T>
where
    T: Proxy + Sync,
{
    async fn bind(&self, cluster_id: u16, destination: Destination) -> Result<u8, Error> {
        self.unicast_zdp(BindReq::new(
            self.ieee_address().await?,
            self.endpoint().into(),
            cluster_id,
            destination,
        ))
        .await
    }

    async fn unbind(&self, cluster_id: u16, destination: Destination) -> Result<u8, Error> {
        self.unicast_zdp(zdp::UnbindReq::new(
            self.ieee_address().await?,
            self.endpoint().into(),
            cluster_id,
            destination,
        ))
        .await
    }
}
