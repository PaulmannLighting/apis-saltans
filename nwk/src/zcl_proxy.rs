use le_stream::ToLeStream;
use zcl::Type;
use zigbee::{Cluster, Command};

use crate::{Error, Proxy};

pub struct ZclProxy<'proxy, T> {
    proxy: &'proxy T,
}

impl<'proxy, T> ZclProxy<'proxy, T> {
    /// Create a new `ZclProxy`.
    pub(crate) const fn new(proxy: &'proxy T) -> Self {
        Self { proxy }
    }
}

impl<T> ZclProxy<'_, T>
where
    T: Proxy + Sync,
{
    /// Send a ZCL command to a specific endpoint on a device.
    pub async fn unicast<C>(
        &self,
        pan_id: u16,
        endpoint: zigbee::Endpoint,
        command: C,
    ) -> Result<(), Error>
    where
        C: Command + ToLeStream,
    {
        self.proxy
            .unicast(
                pan_id,
                endpoint,
                <C as Cluster>::ID,
                // FIXME: The `seq` of `0x00` is a placeholder.
                // It must be replaced with a proper transaction sequence number on the actor's side.
                zcl::Frame::new(Type::ClusterSpecific, true, None, 0x00, command).into(),
            )
            .await
    }
}
