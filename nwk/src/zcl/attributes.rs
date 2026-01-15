use zcl::Global;
use zcl::global::read_attributes::Command;

use crate::proxies::EndpointProxy;
use crate::{Error, Proxy};

/// Trait for managing ZCL attributes.
pub trait Attributes {
    /// Read attributes from the specified cluster.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the read operation fails.
    fn read_attributes(
        &self,
        cluster_id: u16,
        attribute_ids: Box<[u16]>,
    ) -> impl Future<Output = Result<u8, Error>>;
}

impl<T> Attributes for EndpointProxy<'_, T>
where
    T: Proxy + Sync,
{
    async fn read_attributes(
        &self,
        cluster_id: u16,
        attribute_ids: Box<[u16]>,
    ) -> Result<u8, Error> {
        self.zcl()
            .unicast(Command::new(attribute_ids).for_cluster(cluster_id))
            .await
    }
}
