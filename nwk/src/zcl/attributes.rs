use zcl::global::read_attributes::Command;
use zcl::{Global, ReadableAttribute};

use crate::proxies::EndpointProxy;
use crate::{Error, Proxy};

/// Trait for managing ZCL attributes.
pub trait Attributes {
    /// Read attributes from the specified cluster.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if execution of the command failed.
    fn read_raw(
        &self,
        cluster_id: u16,
        attribute_ids: Box<[u16]>,
    ) -> impl Future<Output = Result<u8, Error>> + Send;

    /// Read attributes of a specific cluster.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if execution of the command failed.
    fn read<T>(&self, attributes: &[T]) -> impl Future<Output = Result<u8, Error>> + Send
    where
        T: ReadableAttribute,
    {
        self.read_raw(
            T::ID,
            attributes
                .iter()
                .copied()
                .map(Into::into)
                .collect::<Box<[u16]>>(),
        )
    }
}

impl<T> Attributes for EndpointProxy<'_, T>
where
    T: Proxy + Sync,
{
    async fn read_raw(&self, cluster_id: u16, attribute_ids: Box<[u16]>) -> Result<u8, Error> {
        self.zcl()
            .unicast(Command::new(attribute_ids).for_cluster(cluster_id))
            .await
    }
}
